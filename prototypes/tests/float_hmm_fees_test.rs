//* these test correspond to all examples in pool_hmm.iypnb
//* of the python prototype. Tests HMM mechanism, and fee mechanism

#[cfg(test)]
mod tests {

    use prototypes::cl_pool_float::*;

    #[test]
    fn test_hmm_no_fees() {
        let mut cpmm_pool = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 0.0, 0.0);
        cpmm_pool.deposit("abc", 2.0, 4000.0, 1333_f64.sqrt(), 3000_f64.sqrt());
        let (x_a, x_adj_a, x_fee_a) = cpmm_pool.x_info();
        let (y_a, y_adj_a, y_fee_a) = cpmm_pool.y_info();

        let mut hmm_pool = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 1.5, 0.0);
        hmm_pool.deposit("abc", 2.0, 4000.0, 1333_f64.sqrt(), 3000_f64.sqrt());
        let (x_h, x_adj_h, x_fee_h) = hmm_pool.x_info();
        let (y_h, y_adj_h, y_fee_h) = hmm_pool.y_info();

        // no difference on deposits
        assert_eq!(x_a, x_h);
        assert_eq!(x_adj_a, x_adj_h);
        assert_eq!(x_fee_a, x_fee_h);
        assert_eq!(y_a, y_h);
        assert_eq!(y_adj_a, y_adj_h);
        assert_eq!(y_fee_a, y_fee_h);

        let rez_a = cpmm_pool.execute_swap_from_x(3.0, 1500_f64.sqrt());
        let rez_h = hmm_pool.execute_swap_from_x(3.0, 1500_f64.sqrt());
        // same quantity of X swapped
        assert_eq!(rez_h.recv_amount(), rez_a.recv_amount());
        // hmm gives out less of  asset Y (smaller abs value)
        assert!(rez_h.send_amount().abs() < rez_a.send_amount().abs());
        // same end price for the pool
        assert_eq!(rez_h.end_price(), rez_a.end_price());
        // pool buying X, hmm_pool buys at cheaper avg price
        assert!(rez_h.avg_price() < rez_a.avg_price());
        // y_adj accounts for the difference in Y given out
        assert_eq!(
            rez_h.send_hmm_adj(),
            rez_a.send_amount().abs() - rez_h.send_amount().abs()
        );

        // post reserves are the same. hmm impact in taken out of reserves and put into adj_fee pot
        assert_eq!(hmm_pool.x_info().0, cpmm_pool.x_info().0);
        assert_eq!(hmm_pool.y_info().0, cpmm_pool.y_info().0);
        // global price and tick are the same
        assert_eq!(hmm_pool.glbl_tick(), cpmm_pool.glbl_tick());
        assert_eq!(hmm_pool.glbl_rp(), cpmm_pool.glbl_rp());

        if Pool::ADJ_WHOLE_FILL == 0.0 && !Pool::FLOOR_LIQ {
            assert_eq!(hmm_pool.y_info().1, 300.0419098796583_f64);
            assert_eq!(hmm_pool.glbl_rp(), 1332.937255554048_f64.sqrt());
            assert_eq!(rez_h.avg_price(), 1510.2149015898015_f64);
        }
        if Pool::ADJ_WHOLE_FILL == 1.0e-12 && Pool::FLOOR_LIQ {
            assert_eq!(hmm_pool.y_info().1, 299.9158574252024_f64);
            assert_eq!(hmm_pool.glbl_rp(), 1332.937255554048_f64.sqrt());
            assert_eq!(rez_h.avg_price(), 1510.2149015882912_f64);
        }
        // do another swap, from Y this time
        let res_a = cpmm_pool.execute_swap_from_y(3955.0, 1700_f64.sqrt());
        let res_h = hmm_pool.execute_swap_from_y(3955.0, 1700_f64.sqrt());
        // same quantity of Y swapped
        assert_eq!(res_h.recv_amount(), res_a.recv_amount());
        // hmm gives out less of  asset X (smaller abs value)
        assert!(res_h.send_amount().abs() < res_a.send_amount().abs());
        // same end price for the pool
        assert_eq!(res_h.end_price(), res_a.end_price());
        // pool buying Y (selling X), hmm_pool sell at higher avg price
        assert!(res_h.end_price() > res_a.avg_price());
        // x_adj accounts for the difference in X given out
        assert_eq!(
            res_h.send_hmm_adj(),
            res_a.send_amount().abs() - res_h.send_amount().abs()
        );

        // post reserves, global price and tick are still the same after 2 swaps
        assert_eq!(hmm_pool.x_info().0, cpmm_pool.x_info().0);
        assert_eq!(hmm_pool.y_info().0, cpmm_pool.y_info().0);
        assert_eq!(hmm_pool.glbl_tick(), cpmm_pool.glbl_tick());
        assert_eq!(hmm_pool.glbl_rp(), cpmm_pool.glbl_rp());

        if Pool::ADJ_WHOLE_FILL == 0.0 && !Pool::FLOOR_LIQ {
            assert_eq!(hmm_pool.x_info().1, 0.13387105926908927_f64);
            assert_eq!(hmm_pool.y_info().1, 300.0419098796583_f64); // same as after 1st swap
            assert_eq!(hmm_pool.glbl_rp(), 1991.582634658289_f64.sqrt());
            assert_eq!(res_h.avg_price(), 1724.4121583231515_f64);
        }
        if Pool::ADJ_WHOLE_FILL == 1.0e-12 && Pool::FLOOR_LIQ {
            assert_eq!(hmm_pool.x_info().1, 0.13381481787398464_f64);
            assert_eq!(hmm_pool.y_info().1, 299.9158574252024_f64); // same as after 1st swap
            assert_eq!(hmm_pool.glbl_rp(), 1991.8871664747417_f64.sqrt());
            assert_eq!(res_h.avg_price(), 1724.5093970632417_f64);
        }
    }

    #[test]
    fn test_fees() {
        let x_to_swap = 3_f64;
        let fee_rate = 0.003_f64;

        let mut hmm_nofee = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 1.5, 0.0);
        hmm_nofee.deposit("abc", 2.0, 4000.0, 1333_f64.sqrt(), 3000_f64.sqrt());
        let x_in_pool_n = hmm_nofee.x_info().0;

        let mut hmm_feed = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 1.5, fee_rate);
        hmm_feed.deposit("abc", 2.0, 4000.0, 1333_f64.sqrt(), 3000_f64.sqrt());
        let x_in_pool_f = hmm_feed.x_info().0;
        let pool_liq = hmm_feed.glbl_liq();

        assert_eq!(x_in_pool_f, x_in_pool_n);

        let rez_n = hmm_nofee.execute_swap_from_x(x_to_swap, 1500_f64.sqrt());
        let rez_f = hmm_feed.execute_swap_from_x(x_to_swap, 1500_f64.sqrt());

        if x_to_swap > x_in_pool_n {
            // fee impact: more X received for same amount of Y given out
            assert!(
                rez_f.recv_amount() > rez_n.recv_amount()
                    && rez_f.send_amount().abs() == rez_n.send_amount().abs()
            );
            // same amount of Y given out ==> price impact is same
            assert!(rez_f.end_price() == rez_n.end_price());
            // x_fee accounts for the difference between (x post_swap)  and (amt sent in + x pre_swap)
            assert!(
                x_in_pool_f + rez_f.recv_amount() - (hmm_feed.x_info().0 + rez_f.recv_fee())
                    < rez_f.recv_fee() * 1e-12
            );
            assert_eq!(
                x_in_pool_n + rez_n.recv_amount(),
                hmm_nofee.x_info().0 + rez_n.recv_fee()
            );
        } else {
            // fee impact: same X received for less amount of Y given out
            assert!(
                rez_f.recv_amount() == rez_n.recv_amount()
                    && rez_f.send_amount().abs() < rez_n.send_amount().abs()
            );
            // less amount of Y given out ==> price impact is smaller (falls less)
            assert!(rez_f.end_price() > rez_n.end_price());
            // x_fee accounts for the difference between (x post_swap)  and (amt sent in + x pre_swap)
            //* here small diff due to error in fee_x 'reverse-engineering' (line 774 )
            assert!(
                x_in_pool_f + rez_f.recv_amount() - (hmm_feed.x_info().0 + rez_f.recv_fee())
                    < rez_f.recv_fee() * 1e-12
            );
            assert_eq!(
                x_in_pool_n + rez_n.recv_amount(),
                hmm_nofee.x_info().0 + rez_n.recv_fee()
            );
        }
        // EITHER WAY, pool buying X so with fees, pool buys at cheaper avg price
        assert!(rez_f.avg_price() < rez_n.avg_price());

        // the fee charged is in fee pot
        assert_eq!(hmm_feed.x_info().2, rez_f.recv_fee());
        assert_eq!(hmm_nofee.x_info().2, rez_n.recv_fee());
        // and it is not zero
        assert!(hmm_feed.x_info().2 > 0.0);
        assert!(hmm_nofee.x_info().2 == 0.0);

        assert!((rez_f.recv_fee() / rez_f.recv_amount() - fee_rate).abs() < 1e-8_f64);

        // before withdrawal x_fee and y_adj are in the fee pots
        assert!(hmm_feed.x_info().2 + hmm_feed.y_info().1 > 0.0);

        hmm_feed.withdraw("abc", pool_liq, 1333_f64.sqrt(), 3000_f64.sqrt());

        // after withdrawal, fee pots are empty (transferred to LP along with assets)
        assert!(hmm_feed.x_info().2 + hmm_feed.y_info().1 == 0.0);
        assert_eq!(hmm_feed.position_count(), 0);
    }

    #[test]
    fn test_no_infinite_loop() {
        let mut single = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 0.0, 0.003_f64);
        single.deposit("alice", 2.0, 4000.0, 1333_f64.sqrt(), 3000_f64.sqrt());

        let rez_single = single.execute_swap_from_x(3.0, 1500_f64.sqrt());

        let mut split = Pool::new("ETH", 18, "USDC", 6, 2000_f64.sqrt(), 1, 0.0, 0.003_f64);
        split.deposit("bob", 0.0, 4000.0, 1333_f64.sqrt(), 2000_f64.sqrt());
        split.deposit("carl", 2.0, 0.0, 2000_f64.sqrt(), 3000_f64.sqrt());

        let rez_split = split.execute_swap_from_x(3.0, 1500_f64.sqrt());

        // whether liquidity is provided in one interval or 2 adjacents intervals makes no diff to swap_from_x
        // here deposited amount in x in sligthly different due to:
        //  _ in 'single', liq_x_only and liq_y_only are compared and the min is taken (in this case liq_y_only is taken)
        //  _ in 'split' no comparaison is done so input x & y taken in as given
        // in this case X reserves are slightly different
        // but this doesnt affect the result on a swap_from_x

        assert_eq!(rez_single, rez_split);
    }

    #[test]
    fn test_small_trades() {
        let mut pool = Pool::new("HYS", 12, "USDC", 6, 10000_f64.sqrt(), 1, 0.0, 0.003_f64);
        pool.deposit("aly", 1000.0, 10000000.0, 8000_f64.sqrt(), 12500_f64.sqrt());

        let orig_tick = pool.glbl_tick();
        let orig_p = pool.glbl_rp().powi(2);
        let rez = pool.execute_swap_from_x(1e-11_f64, 0.0);
        let new_tick = pool.glbl_tick();
        let new_p = pool.glbl_rp().powi(2);

        // even tiniest of trades moves the needle (price) and is executed
        assert!(rez.recv_amount() > 0.0);
        assert!(rez.send_amount() < 0.0);
        assert!(orig_tick >= new_tick);
        assert!(orig_p > new_p);
    }
}
