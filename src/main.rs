mod alloc;
mod solutions;

use alloc::MyAlloc;
use ascii_table::AsciiTable;
use humansize::{format_size, BINARY};
use solutions::*;
use stats_alloc::{Region, StatsAlloc};
use std::fmt::Display;
use std::hint::black_box;
use std::time::Instant;

#[global_allocator]
static GLOBAL: StatsAlloc<MyAlloc> = StatsAlloc::new(MyAlloc::new());

const ITERATIONS: u64 = 10_000_000;

struct Row {
    name: &'static str,
    creation_time: u128,
    run_time: u128,
    destroy_time: u128,
    total_time: u128,
    allocs: usize,
    max_memory: usize,
}

struct RowExtra {
    slower_total: String,
    slower_run: String,
    max_memory: String,
}

fn main() {
    let mut data: Vec<Row> = Vec::new();

    macro_rules! bench {
        ($t:ident) => {
            let region = Region::new(&GLOBAL);
            let time = Instant::now();
            let mut obj = $t::DoubleLinkedList::new((ITERATIONS + 1) as usize);
            for index in 0..ITERATIONS {
                obj.add(ITERATIONS - index);
            }
            let creation_time = time.elapsed();

            let stats = region.change();

            let time = Instant::now();
            let sum = black_box(obj.sum_all());
            let run_time = time.elapsed();

            assert_eq!(sum, ITERATIONS * (ITERATIONS + 1) / 2);
            assert_eq!(stats.reallocations, 0);

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
                allocs: stats.allocations,
                max_memory: stats.bytes_allocated - stats.bytes_deallocated,
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
    // bench!(std_map_impl);
    bench!(std_linked_list_impl);
    // bench!(rc_impl);
    bench!(index_impl);
    bench!(handle_impl);

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);
    ascii_table.column(0).set_header("name");
    ascii_table.column(1).set_header("creation");
    ascii_table.column(2).set_header("run");
    ascii_table.column(3).set_header("destroy");
    ascii_table.column(4).set_header("total");
    ascii_table.column(5).set_header("slower(total)");
    ascii_table.column(6).set_header("slower(run)");
    ascii_table.column(7).set_header("no. allocs");
    ascii_table.column(8).set_header("max memory");

    let min_total = data.iter().map(|x| x.total_time).min().unwrap() as f64;
    let min_run = data.iter().map(|x| x.run_time).min().unwrap() as f64;
    let mut extra = Vec::with_capacity(data.len());
    for i in data.iter() {
        extra.push(RowExtra {
            slower_total: format!("{:.02}x", i.total_time as f64 / min_total),
            slower_run: format!("{:.02}x", i.run_time as f64 / min_run),
            max_memory: format_size(i.max_memory, BINARY),
        });
    }

    let it = data
        .iter()
        .enumerate()
        .map(|(index, x)| -> [&dyn Display; 9] {
            [
                &x.name,
                &x.creation_time,
                &x.run_time,
                &x.destroy_time,
                &x.total_time,
                &extra[index].slower_total,
                &extra[index].slower_run,
                &x.allocs,
                &extra[index].max_memory,
            ]
        });
    ascii_table.print(it);
}
