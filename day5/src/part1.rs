use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader, Error, Lines},
};

#[derive(Debug)]
pub struct AdvMapEntry {
  destination_initial: i64,
  origin_initial: i64,
  range: i64,
}

#[derive(Debug)]
pub struct AdvMap {
  from: String,
  to: String,
  entries: Vec<AdvMapEntry>,
}

#[derive(Debug)]
pub struct Input {
  seeds: Vec<i64>,
  maps: HashMap<String, AdvMap>,
}

pub fn solve_p1(input: &Input) -> i64 {
  let mut lowest_location_number: Option<i64> = None;

  for seed in &input.seeds {
      let mut location_number = 0_i64;
      let mut next_map_key = "seed";
      let mut current_map_result = *seed;

      while next_map_key != "location" && next_map_key != "" {
          let current_map = input.maps.get(next_map_key).unwrap();
          next_map_key = current_map.to.as_str();

          let mut found_entry = false;

          for entry in &current_map.entries {
              if current_map_result >= entry.origin_initial
                  && current_map_result < entry.origin_initial + entry.range
              {
                  found_entry = true;
                  current_map_result = entry.destination_initial - entry.origin_initial + current_map_result;
              }

              if found_entry {
                  break;
              }
          }

          if next_map_key == "location" {
              location_number = current_map_result;
          }
      }

      if lowest_location_number.is_none() || lowest_location_number.unwrap() > location_number {
          lowest_location_number = Some(location_number);
      }
  }

  return lowest_location_number.unwrap();
}

pub fn parse_input(raw_lines: Lines<BufReader<File>>) -> Input {
  let mut input = Input {
      seeds: vec![],
      maps: HashMap::new(),
  };

  let lines = raw_lines.filter_map(|l| l.ok()).collect::<Vec<_>>();

  let grouped_lines = lines.split(|l| l == "").collect::<Vec<_>>();

  let (seed_line, map_lines) = grouped_lines.split_first().unwrap();

  let seeds = seed_line
      .get(0)
      .unwrap()
      .split("seeds:")
      .collect::<Vec<_>>()
      .get(1)
      .unwrap()
      .split(" ")
      .filter_map(|s| s.parse::<i64>().ok())
      .collect::<Vec<_>>();

  input.seeds = seeds;

  for raw_map in map_lines {
      let mut map = AdvMap {
          entries: vec![],
          from: "".to_string(),
          to: "".to_string(),
      };

      let (title, entries) = raw_map.split_first().unwrap();
      let title_vec = title
          .split(" map")
          .collect::<Vec<_>>()
          .first()
          .unwrap()
          .split("-to-")
          .collect::<Vec<_>>();
      let from = title_vec.first().unwrap();
      let to = title_vec.get(1).unwrap();
      println!("from {} to {}", from, to);

      map.from = from.to_string();
      map.to = to.to_string();

      for raw_entry in entries {
          let mut entry = AdvMapEntry {
              range: 0,
              origin_initial: 0,
              destination_initial: 0,
          };

          let raw_entry_numbers = raw_entry
              .split(" ")
              .flat_map(|s| s.parse::<i64>().ok())
              .collect::<Vec<_>>();

          println!("{:?}", raw_entry_numbers);

          let destination_initial = raw_entry_numbers.get(0).unwrap();
          let origin_initial = raw_entry_numbers.get(1).unwrap();
          let range = raw_entry_numbers.get(2).unwrap();

          entry.destination_initial = *destination_initial;
          entry.origin_initial = *origin_initial;
          entry.range = *range;

          map.entries.push(entry);
      }

      input.maps.insert(map.from.clone(), map);
  }

  return input;
}
