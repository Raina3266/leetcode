pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len()];
    
    dp[0] = cost[0];
    dp[1] = std::cmp::min(cost[0], cost[1]);

    if cost.len() == 2 {
        return dp[1];
    }

    for i in 2..cost.len() {

        dp[i] = std::cmp::min(cost[i-2] + cost[i], cost[i-1]);
    }

    dp.pop().unwrap()
}