use yml_parser::parser::LoadBalancerConfig;

pub fn add(left: usize, right: usize) -> usize {
    println!("LB CORE");
    left + right
}

pub fn initialize_load_balancer(lb_config: LoadBalancerConfig) {
   println!("Initialize Load Balancer Via Config : {:?} ", lb_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
