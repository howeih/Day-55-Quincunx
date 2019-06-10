extern crate gnuplot;
extern crate ndarray;
extern crate rand;

use std::{thread, time};
use gnuplot::{Figure};
use ndarray::{Array1, Array2, ArrayView1};
use rand::seq::SliceRandom;

fn update_buckets(buckets: &mut Array1<f64>, bucket_update_pos: Vec<usize>) {
    for p in bucket_update_pos {
        buckets[(p)] += 1.;
    }
}

fn find_ones(y: ArrayView1<f64>) -> Vec<usize> {
    let mut one_pos = Vec::<usize>::new();
    for (k, v) in y.iter().enumerate() {
        if *v == 1. {
            one_pos.push(k);
        }
    }
    one_pos
}

fn get_bucket_update_pos(x1: ArrayView1<f64>, y_one_pos: Vec<usize>, quincunx_size: usize) -> Vec<usize> {
    let mut bucket_pos = Vec::<usize>::new();
    for p in y_one_pos {
        bucket_pos.push((x1[p] + quincunx_size as f64) as usize);
    }
    bucket_pos
}

fn update_quincunx(quincunx_size: usize, balls: &mut Array2<f64>, buckets: &mut Array1<f64>) {
    let balls_t = balls.t();
    // update buckets
    let y_one_pos = find_ones(balls_t.row(2));
    let bucket_update_pos = get_bucket_update_pos(balls_t.row(1), y_one_pos, quincunx_size);
    update_buckets(buckets, bucket_update_pos);

    // update y
    balls.column_mut(2).iter_mut().for_each(|y| *y -= 1.);
    balls.column_mut(2).iter_mut().for_each(|y| if *y <= 0. {
        *y = quincunx_size as f64;
    });

    // update x
    let mut y_quincunex_size = Vec::<usize>::new();
    for (i, y) in balls.column(2).iter().enumerate() {
        if *y == quincunx_size as f64 {
            y_quincunex_size.push(i);
        }
    }
    for y in y_quincunex_size {
        balls.column_mut(1)[(y as usize)] = 0.;
    }
    let x1: Array1<f64> = balls.column(1).iter().cloned().collect();
    for (i, x1_itr) in x1.iter().enumerate() {
        balls.column_mut(0)[(i)] = *x1_itr;
    }
    balls.column_mut(1).iter_mut().for_each(|x| {
        let vs = [-1., 1.];
        let rand_choose = vs.choose(&mut rand::thread_rng()).unwrap();
        *x += rand_choose;
    });
}

fn animate_quincunx(time: f64, balls: &Array2<f64>) -> (Array1<f64>, Array1<f64>) {
    let balls_t = balls.t();
    let x0 = balls_t.row(0);
    let x1 = balls_t.row(1);
    let y = balls_t.row(2);
    let mut u = Array1::<f64>::zeros( balls.rows());
    u.iter_mut().for_each(|u| *u=1.);
    let x = (1. - time) * &x0 + &x1 * time;
    let y = &y * (1. - time) + (&y - &u) * time;
    (x, y)
}


fn init(quincunx_size: usize) -> (Array2<f64>, Array1<f64>) {
    let mut balls = Array2::<f64>::zeros((quincunx_size - 1, 3));
    for i in 0..quincunx_size - 1 {
        balls[(i, 2)] = quincunx_size as f64 + 1. + i as f64;
    }
    let buckets = Array1::<f64>::zeros(2 * quincunx_size + 1);
    (balls, buckets)
}

fn main() {
    let quincunx_size = 30;
    let mut fg = Figure::new();
    let (mut balls, mut buckets) = init(quincunx_size);
    let (mut x, mut y) = animate_quincunx(1., &balls);
    for i in 0..quincunx_size * 5 * 30 {
        let time = ((i as f64 / 5. % 1.) * 10.0 as f64).round() as f64 / 10.;
        if time == 0. {
            update_quincunx(quincunx_size, &mut balls, &mut buckets);
        }
        let q = animate_quincunx(time, &balls);
        x = q.0;
        y = q.1;
        thread::sleep(time::Duration::from_millis(100));
        fg.clear_axes();
        fg.axes2d().points(&x, &y, &[]);
        fg.show();
    }
}
