use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Route {
    pub broker: String,
    pub queue: String
}

#[derive(Debug)]
pub struct RoutingCondition {
    pub field: String,
    pub value: String,  // TODO 0.0.2 value should be of any type
//  operator: ??
}

#[derive(Debug)]
pub struct RouteDescription {
    pub conditions: Vec<RoutingCondition>,
    pub route_to: Vec<Route>
}

pub fn route(routes_description: &Vec<RouteDescription>, message: &HashMap<String, String>) -> Vec<Route> {
    routes_description
        .iter()
        .filter_map(|route_description|{
            for condition in &route_description.conditions {
                match message.get(&condition.field) {
                    Some(value) => {
                        if *value != condition.value {
                            return None;
                        }
                    },
                    None => {
                        return None;
                    },
                };
            };
            Some(route_description.route_to.clone())
        })
        .flat_map(|routes|{
            routes
        })
        .collect::<Vec<_>>()
}

mod test_route_functionality {
    use super::{RouteDescription, RoutingCondition, Route, route};
    use super::HashMap;
    fn generate_routes_description() -> Vec<RouteDescription> {
        let mut routes_description = vec![];
        for i in 0..4 {
            let route_description = RouteDescription {
                conditions: vec![
                    RoutingCondition{
                        field: "type".to_string(),
                        value: "addUser".to_string(),
                    }],
                route_to: vec![
                    Route{
                        broker: format!("rabbitmq{}", i),
                        queue: "user".to_string()
                    }
                ]
            };
            routes_description.push(route_description);
        }
        routes_description
    }

    #[test]
    fn route_function_should_return_routes_on_match() {
        let routes_description = generate_routes_description();
        let mut message = HashMap::new();
        message.insert("type".to_string(), "addUser".to_string());
        let routes = route(&routes_description, &message);

        assert_eq!(routes.len(), 4);
    }

    #[test]
    fn route_function_should_not_return_routes_on_not_match() {
        let routes_description = generate_routes_description();
        let mut message = HashMap::new();
        message.insert("type".to_string(), "deleteUser".to_string());
        let routes = route(&routes_description, &message);

        assert_eq!(routes.len(), 0);
    }
}