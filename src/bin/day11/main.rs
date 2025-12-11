use std::collections::HashMap;
use aoc2025::utils;

pub fn get_details() -> utils::ExecDetails {
    return utils::ExecDetails {
        day: 11,
        sample: include_str!("sample").to_string(),
        sample2: Some(include_str!("sample2").to_string()),
        input: include_str!("input").to_string(),
        task1_function: task1,
        task2_function: task2,
        task1_sample_expected: 5,
        task1_input_expected: 607,
        task2_sample_expected: 2,
        task2_input_expected: 506264456238938,
    };
}

fn main() {
    utils::exec(&get_details());
}

fn find_paths_recur(
    current: &String,
    graph: &HashMap<String, Vec<String>>,
    paths_found: u32
) -> u32
{
    let mut total_paths_found = paths_found;

    for neighbour in graph.get(current).unwrap() {
        if(neighbour == "out")
        {
            total_paths_found += 1;
        }
        else {
            total_paths_found = find_paths_recur(
                &neighbour,
                graph,
                total_paths_found
            )
        }
    }

    return total_paths_found;
}

pub fn task1(input: &String) -> i64 {

    let mut device_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let colon_index = line.find(":").unwrap() as usize;
        let source_device = line[0..colon_index].to_string();

        let target_devices_str = line[colon_index + 1..].trim().to_string();
        let target_devices = target_devices_str.split(" ").map(|part| part.to_string()).collect();

        device_map.insert(source_device, target_devices);
    }

    let paths = find_paths_recur(&"you".to_string(), &device_map, 0);

    return paths as i64;
}

fn count_paths(
    current: &String,
    target: &String,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64
{
    if(current == target)
    {
        return 1;
    }

    let cached_value = cache.get(current);
    if(cached_value.is_some())
    {
        return *cached_value.unwrap();
    }

    let mut sum = 0;
    let neighbours = graph.get(current);
    if(neighbours.is_some())
    {
        for neighbour in neighbours.unwrap() {
            sum += count_paths(
                neighbour,
                target,
                graph,
                cache
            )
        }
    }

    cache.insert(current.clone(), sum);
    return sum;
}

pub fn task2(input: &String) -> i64 {
    let mut device_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let colon_index = line.find(":").unwrap() as usize;
        let source_device = line[0..colon_index].to_string();

        let target_devices_str = line[colon_index + 1..].trim().to_string();
        let target_devices = target_devices_str.split(" ").map(|part| part.to_string()).collect();

        device_map.insert(source_device, target_devices);
    }

    //Observed that FFT is always before DAC in the DAG, if I needed to I'd just need
    //to reverse which values I was looking at to calculated if DAC was first. But I'm lazy

    let paths_from_svr_to_fft = count_paths(
        &"svr".to_string(),
        &"fft".to_string(),
        &device_map,
        &mut HashMap::new(),
    ) as i64;

    let paths_from_fft_to_dac = count_paths(
        &"fft".to_string(),
        &"dac".to_string(),
        &device_map,
        &mut HashMap::new(),
    ) as i64;

    let paths_from_dac_to_out = count_paths(
        &"dac".to_string(),
        &"out".to_string(),
        &device_map,
        &mut HashMap::new(),
    ) as i64;

    let paths_to_dac_through_fft = paths_from_svr_to_fft * paths_from_fft_to_dac;
    let paths_to_output_through_dac_and_fft = paths_to_dac_through_fft * paths_from_dac_to_out;
    return paths_to_output_through_dac_and_fft;
}