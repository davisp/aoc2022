use std::collections::BinaryHeap;

use crate::errors::AoCError;

#[derive(Clone, Debug)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    op: fn(&u64) -> u64,
    test: fn(&u64) -> bool,
    dest: Vec<u64>,
    inspections: u64,
}

impl Monkey {
    fn sample_input() -> Vec<Monkey> {
        vec![
            Monkey {
                id: 0,
                items: vec![79, 98],
                op: |&old| old * 19,
                test: |&val| val % 23 == 0,
                dest: vec![2, 3],
                inspections: 0,
            },
            Monkey {
                id: 1,
                items: vec![54, 65, 75, 74],
                op: |&old| old + 6,
                test: |&val| val % 19 == 0,
                dest: vec![2, 0],
                inspections: 0,
            },
            Monkey {
                id: 2,
                items: vec![79, 60, 97],
                op: |&old| old * old,
                test: |&val| val % 13 == 0,
                dest: vec![1, 3],
                inspections: 0,
            },
            Monkey {
                id: 3,
                items: vec![74],
                op: |&old| old + 3,
                test: |&val| val % 17 == 0,
                dest: vec![0, 1],
                inspections: 0,
            },
        ]
    }

    fn input() -> Vec<Monkey> {
        vec![
            Monkey {
                id: 0,
                items: vec![54, 82, 90, 88, 86, 54],
                op: |old| old * 7,
                test: |val| val % 11 == 0,
                dest: vec![2, 6],
                inspections: 0,
            },
            Monkey {
                id: 1,
                items: vec![91, 65],
                op: |old| old * 13,
                test: |val| val % 5 == 0,
                dest: vec![7, 4],
                inspections: 0,
            },
            Monkey {
                id: 2,
                items: vec![62, 54, 57, 92, 83, 63, 63],
                op: |old| old + 1,
                test: |val| val % 7 == 0,
                dest: vec![1, 7],
                inspections: 0,
            },
            Monkey {
                id: 3,
                items: vec![67, 72, 68],
                op: |old| old * old,
                test: |val| val % 2 == 0,
                dest: vec![0, 6],
                inspections: 0,
            },
            Monkey {
                id: 4,
                items: vec![68, 89, 90, 86, 84, 57, 72, 84],
                op: |old| old + 7,
                test: |val| val % 17 == 0,
                dest: vec![3, 5],
                inspections: 0,
            },
            Monkey {
                id: 5,
                items: vec![79, 83, 64, 58],
                op: |old| old + 6,
                test: |val| val % 13 == 0,
                dest: vec![3, 0],
                inspections: 0,
            },
            Monkey {
                id: 6,
                items: vec![96, 72, 89, 70, 88],
                op: |old| old + 4,
                test: |val| val % 3 == 0,
                dest: vec![1, 2],
                inspections: 0,
            },
            Monkey {
                id: 7,
                items: vec![79],
                op: |old| old + 8,
                test: |val| val % 19 == 0,
                dest: vec![4, 5],
                inspections: 0,
            },
        ]
    }
}

fn do_round(factor: u64, monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        for item in monkeys[i].items.clone().iter() {
            let worry = (monkeys[i].op)(item) % factor;
            let idx = if (monkeys[i].test)(&worry) { 0 } else { 1 };
            let dest = monkeys[i].dest[idx];
            monkeys[dest as usize].items.push(worry);
            monkeys[i].items.clear();
            monkeys[i].inspections += 1;
        }
    }
}

pub fn run(input: String) -> Result<(), AoCError> {
    let mut monkeys = if input.ends_with("_sample") {
        Monkey::sample_input()
    } else {
        Monkey::input()
    };

    let factor = if input.ends_with("_sample") {
        96577
    } else {
        9699690
    };

    for _ in 0..10000 {
        do_round(factor, &mut monkeys);
    }

    let mut heap = BinaryHeap::new();
    for m in monkeys.iter() {
        println!("Inspections: {} {}", m.id, m.inspections);
        heap.push(m.inspections);
    }

    let a = heap.pop().unwrap();
    let b = heap.pop().unwrap();
    println!("Monkey Business! {}", a * b);

    Ok(())
}
