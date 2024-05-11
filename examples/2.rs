// Napsack

use evolutionary_algorithm::{EAServer, Evaluation, Individual};

struct Item {
    value: i64,
    weight: i64,
    stock: i64,
}

fn main() {
    let seed = 0;
    let epoch = 1000;
    let indi_num = 50;
    let indi_len = 10;
    let vector_num = 1;
    let f_scale = 0.5;
    let own_ratio = 0.5;

    let mut ea_server = EAServer::new_with_random(indi_num, indi_len, seed);

    let weight_constraint = 1000;
    let items = vec![
        Item {
            value: 1,
            weight: 10,
            stock: 100,
        },
        Item {
            value: 2,
            weight: 9,
            stock: 100,
        },
        Item {
            value: 3,
            weight: 8,
            stock: 100,
        },
        Item {
            value: 4,
            weight: 7,
            stock: 100,
        },
        Item {
            value: 5,
            weight: 6,
            stock: 100,
        },
        Item {
            value: 6,
            weight: 5,
            stock: 100,
        },
        Item {
            value: 7,
            weight: 4,
            stock: 100,
        },
        Item {
            value: 8,
            weight: 3,
            stock: 100,
        },
        Item {
            value: 9,
            weight: 2,
            stock: 100,
        },
        Item {
            value: 10,
            weight: 1,
            stock: 100,
        },
    ];

    fn convert(indi: &Individual, items: &Vec<Item>) -> Vec<f64> {
        items.iter().zip(indi.iter()).map(|(item, gene)| (gene * item.stock as f64).round()).collect()
    }

    fn evaluate(indi: &Individual, items: &Vec<Item>, weight_constraint: i64) -> Evaluation{
        let mut value = 0.0;
        let mut weight = 0.0;
        let cnts = convert(indi, items);

        for (item, cnt) in items.iter().zip(cnts.iter()) {
            value += item.value as f64 * cnt;
            weight += item.weight as f64 * cnt;
        }
        if weight <= weight_constraint as f64{
            weight = 0.0;
        } else {
            weight = -weight;
        }
        // weight制約を満たすことが最重要、その上でvalueをできるだけ大きく。
        let eval = vec![weight as f64, value as f64];
        eval
    }

    for _ in 0..epoch {
        for i in 0..indi_num {
            let indi = ea_server.de_rand_x_bin(i, vector_num, f_scale, own_ratio);
            let eval = evaluate(&indi, &items, weight_constraint);
            ea_server.may_update(i, indi, eval);
        }
    }

    dbg!(weight_constraint);
    println!("best value = {:?}", ea_server.get_best().1);
    dbg!(convert(ea_server.get_best().0, &items));
    dbg!(ea_server.get_cnt_no_update());
}
