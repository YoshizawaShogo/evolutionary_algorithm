use evolutionary_algorithm::EAServer;

fn main() {
    let seed = 0;
    let epoch = 10000;
    let indi_num = 10;
    let indi_len = 10;
    let vector_num = 1;
    let f_scale = 0.5;
    let own_ratio = 0.5;

    let mut ea_server = EAServer::new_with_random(indi_num, indi_len, seed);
    for _ in 0..epoch {
        for i in 0..indi_num {
            let indi = ea_server.de_rand_x_bin(i, vector_num, f_scale, own_ratio);
            let eval = vec![indi.iter().map(|&x| -(0.5-x).powi(2)).sum::<f64>()];
            ea_server.may_update(i, indi, eval);
        }
    }

    dbg!(ea_server.get_best());
    dbg!(ea_server.get_cnt_no_update());
}