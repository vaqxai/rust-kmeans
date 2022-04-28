mod math;
mod observation;

use std::{fs::File, io::{self, BufRead}};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(short, long)]
    dataset: String,

    #[clap(short, long, default_value_t = 3)]
    k: usize,

}

fn assign_random_clusters(observations: &mut Vec<observation::Observation>, k: usize) {
    for observation in observations {
        observation.cluster = Some(rand::random::<usize>() % k + 1);
    }
}

fn get_centroid(observations: &Vec<observation::Observation>, cluster: usize) -> observation::Observation {
    let mut centroid = observation::Observation::new(vec![0.0; observations[0].position.len()], "centroid".to_owned());
    centroid.cluster = Some(cluster);
    let mut count = 0;

    for observation in observations {
        if observation.cluster == Some(cluster) {
            for i in 0..observation.position.len() {
                centroid.position[i] += observation.position[i];
            }
            count += 1;
        }
    }

    for i in 0..centroid.position.len() {
        centroid.position[i] /= count as f64;
    }

    centroid
}

fn get_all_centroids(observations: &Vec<observation::Observation>, k: usize) -> Vec<observation::Observation> {
    let mut centroids = vec![];

    for i in 1..k+1 {
        centroids.push(get_centroid(observations, i));
    }

    centroids
}

fn get_nearest_centroid(observation: &observation::Observation, centroids: &Vec<observation::Observation>) -> usize {
    let mut nearest_centroid = 0;
    let mut min_dist = math::EuclidDist::dist(&observation.position, &centroids[0].position).unwrap();

    for i in 1..centroids.len() {
        let dist = math::EuclidDist::dist(&observation.position, &centroids[i].position).unwrap();
        if dist < min_dist {
            min_dist = dist;
            nearest_centroid = i;
        }
    }

    nearest_centroid
}

fn get_unique_labels(observations: &Vec<&observation::Observation>) -> Vec<String> {
    let mut labels = vec![];

    for observation in observations {
        if !labels.contains(&observation.label) {
            labels.push(observation.label.clone());
        }
    }

    labels
}

fn main() {

    let args = Args::parse();

    // init data

    let f = match File::open(&args.dataset) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut observations: Vec<observation::Observation> = Vec::new();

    for line in io::BufReader::new(f).lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            continue;
        }

        let position = parts.get(0).unwrap().split(",").map(|x| x.parse::<f64>().unwrap()).collect();

        let label = parts.get(1).unwrap_or(&"None").to_owned().to_owned();
        observations.push(observation::Observation::new(position, label));
    }

    println!("{:#?}", observations);

    // init clusters

    let k = args.k;
    assign_random_clusters(&mut observations, k);

    println!("{:#?}", observations);

    // find initial centroids

    let mut centroids: Vec<observation::Observation> = Vec::new();

    for i in 1..=k {
        centroids.push(get_centroid(&observations, i));
    }

    println!("{:#?}", centroids);

    // run k-means

    let mut changed = true;

    while changed {
        changed = false;

        for observation in observations.iter_mut() {
            let nearest_centroid = get_nearest_centroid(observation, &centroids);
            if observation.cluster != Some(nearest_centroid + 1) {
                observation.cluster = Some(nearest_centroid + 1);
                changed = true;
            }
        }

        // calculate mean distances

        let mut sum = 0.0;

        for observation in observations.iter() {
            sum += math::EuclidDist::dist(&observation.position, &centroids[observation.cluster.unwrap() - 1].position).unwrap();
        }

        println!("");
        println!("Sum of mean centroid distances: {}", sum);

        // calculate "clarity"

        println!("Clarity");

        for i in 1..k+1 {

            println!(" Cluster {}:", i);

            let labels = get_unique_labels( &observations.iter().filter(|x| x.cluster == Some(i)).collect() );
            
            for label in labels {
                let count = observations.iter().filter(|x| x.cluster == Some(i) && x.label == label).count();
                let total = observations.iter().filter(|x| x.cluster == Some(i)).count();
                println!(" {} {}%", label, count as f64/total as f64 * 100.0);
            }
        }

        centroids = get_all_centroids(&observations, k);

        println!("");

    }

    // Final observations

    println!("{:#?}", observations);

}
