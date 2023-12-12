use std::{
    borrow::BorrowMut,
    cmp,
    fs::File,
    io::{BufReader, Lines},
};





#[derive(Clone, Debug)]
struct TransformationMap {
    lower_bound: i64,
    upper_bound: i64,
    transformation_amount: i64,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct SeedRange {
    start: i64,
    end: i64,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct RangeWithOperation {
    range: SeedRange,
    total_transformation: i64,
}

#[derive(Clone, Debug)]
struct RangesWithOperation {
    ranges: Vec<RangeWithOperation>,
}

impl RangesWithOperation {
    fn process(&mut self, transformation_maps: &Vec<TransformationMap>) {
        println!("***************");
        println!("***************");
        println!("PROCESSING");
        println!("{:?}", transformation_maps);

        let mut new_ranges = Vec::new();
        println!("{:?}", self.ranges);
        let mut ranges_left_to_compare = self.ranges.clone();
        println!("{:?}", ranges_left_to_compare);

        for map in transformation_maps {
            let mut remaining_transformation_option = Some(map.clone());

            println!("--------------------------");
            println!("TRANSFORMATION MAP");
            println!("{:?}", map);
            println!("{:?}", ranges_left_to_compare);
            println!("{:?}", remaining_transformation_option);

            match remaining_transformation_option {
                Some(remaining_transformation) => {
                    for seed_range in ranges_left_to_compare.clone().iter() {
                        println!("SEED RANGE");
                        println!("{:?}", seed_range);

                        let seed_range_ref_index = ranges_left_to_compare
                            .iter()
                            .position(|r| r == seed_range)
                            .unwrap();

                        let is_lower_bound_above_range =
                            remaining_transformation.lower_bound > seed_range.range.end;
                        let is_upper_bound_below_range =
                            remaining_transformation.upper_bound < seed_range.range.start;

                        let overlap_lower_bound =
                            cmp::max(remaining_transformation.lower_bound, seed_range.range.start);
                        let overlap_upper_bound =
                            cmp::min(remaining_transformation.upper_bound, seed_range.range.end);

                        let passthru_seed_range: Option<RangeWithOperation> =
                            if seed_range.range.start < remaining_transformation.lower_bound {
                                let extra_addition = if remaining_transformation.lower_bound
                                    <= seed_range.range.end
                                {
                                    1
                                } else {
                                    0
                                };

                                let end = cmp::min(
                                    remaining_transformation.lower_bound,
                                    seed_range.range.end,
                                ) - extra_addition;

                                Some(RangeWithOperation {
                                    range: SeedRange {
                                        start: seed_range.range.start,
                                        end,
                                    },
                                    total_transformation: seed_range.total_transformation,
                                })
                            } else {
                                None
                            };

                        let transformed_seed_range: Option<RangeWithOperation> =
                            if !is_upper_bound_below_range && !is_lower_bound_above_range {
                                Some(RangeWithOperation {
                                    range: SeedRange {
                                        start: overlap_lower_bound,
                                        end: overlap_upper_bound,
                                    },
                                    total_transformation: seed_range.total_transformation
                                        + remaining_transformation.transformation_amount,
                                })
                            } else {
                                None
                            };

                        let leftover_seed_range: Option<RangeWithOperation> =
                            if seed_range.range.end > remaining_transformation.upper_bound {
                                let extra_addition = if remaining_transformation.upper_bound
                                    >= seed_range.range.start
                                {
                                    1
                                } else {
                                    0
                                };

                                Some(RangeWithOperation {
                                    range: SeedRange {
                                        start: cmp::max(
                                            seed_range.range.start,
                                            remaining_transformation.upper_bound,
                                        ) + extra_addition,
                                        end: seed_range.range.end,
                                    },
                                    total_transformation: seed_range.total_transformation,
                                })
                            } else {
                                None
                            };

                        if remaining_transformation.upper_bound > overlap_upper_bound {
                            remaining_transformation_option = Some(TransformationMap {
                                transformation_amount: remaining_transformation
                                    .transformation_amount,
                                upper_bound: remaining_transformation.upper_bound,
                                lower_bound: overlap_upper_bound + 1,
                            })
                        } else {
                            remaining_transformation_option = None;
                        }

                        println!(
                            "-- changed reimaining transformation {:?}",
                            remaining_transformation_option
                        );

                        match passthru_seed_range {
                            Some(range) => {
                                println!("--- setting passtrhu {:?}", range);
                                new_ranges.push(range)
                            }
                            None => {}
                        }

                        match transformed_seed_range {
                            Some(range) => {
                                println!("--- setting transformed {:?}", range);
                                new_ranges.push(range)
                            }
                            None => {}
                        }

                        match leftover_seed_range.clone() {
                            Some(range) => {
                                println!("--- leftover found {:?}", range);

                                let mut new_ranges_left_to_compare = ranges_left_to_compare.clone();
                                let edit_range = new_ranges_left_to_compare
                                    .get_mut(seed_range_ref_index)
                                    .unwrap();

                                edit_range.range = range.range;
                                edit_range.total_transformation = range.total_transformation;
                                ranges_left_to_compare = new_ranges_left_to_compare;
                            }
                            None => {
                                println!("--- no leftover");

                                let mut new_ranges_left_to_compare = ranges_left_to_compare.clone();
                                new_ranges_left_to_compare.remove(seed_range_ref_index);
                                ranges_left_to_compare = new_ranges_left_to_compare;
                            }
                        }

                        if leftover_seed_range.is_some() {
                            println!("BREAK BREAK BREAK BREAK BREAK BREAK BREAK ");
                            break;
                        }

                        println!("overlaps {}, {}", overlap_lower_bound, overlap_upper_bound);
                    }
                }
                None => continue,
            }
        }

        println!("*** leftovers");
        println!("*** {:?}", ranges_left_to_compare);
        new_ranges.append(ranges_left_to_compare.as_mut());

        for new_range in new_ranges.iter_mut() {
            new_range.range.start = new_range.range.start + new_range.total_transformation;
            new_range.range.end = new_range.range.end + new_range.total_transformation;
            new_range.total_transformation = 0;
        }

        new_ranges.sort_by(|a,b|a.range.start.cmp(&b.range.start));

        self.ranges = new_ranges;

        println!("NEW RANGES");
        println!("{:?}", self.ranges);
    }


    fn get_lowest_number(&self) -> i64 {
        self.ranges.first().unwrap().range.start
    }
}

#[derive(Clone, Debug)]
pub struct Part2Input {
    seed_ranges: Vec<SeedRange>,
    transformation_maps: Vec<Vec<TransformationMap>>,
}

pub fn solve_p2(input: &Part2Input) -> i64 {
    println!("solving");
    let mut solution = RangesWithOperation {
        ranges: input
            .seed_ranges
            .iter()
            .map(|r| RangeWithOperation {
                range: r.clone(),
                total_transformation: 0,
            })
            .collect::<Vec<_>>(),
    };

    for map in &input.transformation_maps {
        solution.process(map);
    }

    solution.get_lowest_number()
}

pub fn parse_input_2(raw_lines: Lines<BufReader<File>>) -> Part2Input {
    let mut input = Part2Input {
        seed_ranges: vec![],
        transformation_maps: vec![],
    };

    let lines = raw_lines.filter_map(|l| l.ok()).collect::<Vec<_>>();

    let grouped_lines = lines.split(|l| l == "").collect::<Vec<_>>();

    let (seed_line, map_lines) = grouped_lines.split_first().unwrap();

    let seed_numbers = seed_line
        .get(0)
        .unwrap()
        .split("seeds:")
        .collect::<Vec<_>>()
        .get(1)
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let mut seed_ranges: Vec<SeedRange> = Vec::new();

    for seed_number in seed_numbers {
        let last_range = seed_ranges.last_mut();

        if last_range.is_some() && last_range.as_ref().unwrap().end == -1 {
            let last_range_ref = last_range.unwrap();
            last_range_ref.end = last_range_ref.start - 1 + seed_number;
        } else {
            seed_ranges.push(SeedRange {
                start: seed_number,
                end: -1,
            })
        }
    }

    let mut transformations: Vec<Vec<TransformationMap>> = Vec::new();

    for raw_map in map_lines {
        let mut transformation_maps: Vec<TransformationMap> = Vec::new();

        let (_, entries) = raw_map.split_first().unwrap();

        for raw_entry in entries {
            let mut map: TransformationMap = TransformationMap {
                lower_bound: -1,
                upper_bound: -1,
                transformation_amount: 0,
            };

            let raw_entry_numbers = raw_entry
                .split(" ")
                .flat_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>();

            println!("{:?}", raw_entry_numbers);

            let destination_initial = raw_entry_numbers.get(0).unwrap();
            let origin_initial = raw_entry_numbers.get(1).unwrap();
            let range = raw_entry_numbers.get(2).unwrap();

            map.lower_bound = *origin_initial;
            map.upper_bound = origin_initial + range - 1;
            map.transformation_amount = destination_initial - origin_initial;

            transformation_maps.push(map);
        }

        transformation_maps.sort_by(|a, b| a.lower_bound.cmp(&b.lower_bound));

        transformations.push(transformation_maps);
    }

    seed_ranges.sort_by(|a, b| a.start.cmp(&b.start));

    input.seed_ranges = seed_ranges;
    input.transformation_maps = transformations;

    input
}
