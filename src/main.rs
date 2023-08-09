mod solutions;

use ascii_table::AsciiTable;
use solutions::*;
use std::fmt::Display;
use std::hint::black_box;
use std::time::Instant;

const ITERATIONS: u64 = 100_000;

struct Row {
    name: &'static str,
    creation_time: u128,
    run_time: u128,
    destroy_time: u128,
    total_time: u128,
}

fn main() {
    let mut data: Vec<Row> = Vec::new();

    macro_rules! bench {
        ($t:ident) => {
            let time = Instant::now();
            let mut obj = $t::DoubleLinkedList::new();
            for index in 0..ITERATIONS {
                obj.add(ITERATIONS - index);
            }
            let creation_time = time.elapsed();

            let time = Instant::now();
            let sum = black_box(obj.sum_all());
            let run_time = time.elapsed();

            assert_eq!(sum as i64, ITERATIONS as i64 * (ITERATIONS as i64 + 1) / 2);

            let time = Instant::now();
            black_box(drop(obj));
            let destroy_time = time.elapsed();

            let total_time = creation_time + run_time + destroy_time;
            data.push(Row {
                name: stringify!($t),
                creation_time: creation_time.as_millis(),
                run_time: run_time.as_millis(),
                destroy_time: destroy_time.as_millis(),
                total_time: total_time.as_millis(),
            });

            // println!(
            //     "{}:
            //     creation: {:?}
            //     run     : {:?}
            //     destroy : {:?}
            //     total  :  {:?}",
            //     stringify!($t),
            //     creation_time,
            //     run_time,
            //     destroy_time,
            //     total_time
            // );
        };
    }

    bench!(nonnull_impl);
    bench!(slotmap_impl);
    bench!(std_linked_list_impl);
    bench!(rc_impl);
    bench!(index_impl);

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(80);
    ascii_table.column(0).set_header("name");
    ascii_table.column(1).set_header("creation");
    ascii_table.column(2).set_header("run");
    ascii_table.column(3).set_header("destroy");
    ascii_table.column(4).set_header("total");
    ascii_table.column(5).set_header("slower");

    let min_total = data.iter().map(|x| x.total_time).min().unwrap() as f64;
    let mut slower = Vec::with_capacity(data.len());
    for i in data.iter() {
        slower.push(format!("{:.02}x", i.total_time as f64 / min_total));
    }

    let it = data
        .iter()
        .enumerate()
        .map(|(index, x)| -> [&dyn Display; 6] {
            [
                &x.name,
                &x.creation_time,
                &x.run_time,
                &x.destroy_time,
                &x.total_time,
                &slower[index],
            ]
        });
    ascii_table.print(it);
}
