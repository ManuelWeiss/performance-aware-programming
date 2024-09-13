
const EPSILON: f64 = 0.0000001;

fn get_next_number(it: &mut impl Iterator<Item = char>) -> f64 {
    forward_to_colon(it);
    parse_number(it)
}

fn forward_to_colon(it: &mut impl Iterator<Item = char>) {
    while let Some(c) = it.next() {
        if c == ':' {
            break;
        }
    }
}

fn parse_number(it: &mut impl Iterator<Item = char>) -> f64 {
    let mut sb = String::new();
    while let Some(c) = it.next() {
        if c == ',' || c == '}' || c == ']' {
            return sb.parse().unwrap();
        }
        sb.push(c);
    }
    sb.parse().unwrap()
}

pub fn parse_input(it: &mut impl Iterator<Item = char>) -> f64 {
    while let Some(c) = it.next() {
        if c == '[' {
            break;
        }
    }

    let mut sum = 0.0;
    while let Some(c) = it.next() {
        if c == ']' {
            return sum;
        }
        let x0 = get_next_number(it);
        let y0 = get_next_number(it);
        let x1 = get_next_number(it);
        let y1 = get_next_number(it);
        let d = get_next_number(it);
        let dist = haversine_distance(x0, y0, x1, y1);
        if (dist - d).abs() > EPSILON {
            println!("Distance mismatch: {} != {}", dist, d);
        }
        sum += dist;
    }
    sum
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Radius of the earth in km
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) +
        lat1.to_radians().cos() * lat2.to_radians().cos() *
            (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c // Distance in km
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_number_test() {
        let mut input_iterator = r#"{"x0":-64.356,"y0":55.157,"x1":34.008,"y1":-82.643}"#.chars();

        let number1 = get_next_number(&mut input_iterator);
        let number2 = get_next_number(&mut input_iterator);
        assert_eq!(number1, -64.356);
        assert_eq!(number2, 55.157);
    }

    #[test]
    fn parse_input_test() {
        let mut input_iterator = r#"{"pairs":[{"x0":-64.356,"y0":55.157,"x1":34.008,"y1":-82.643,"d":15606.488392541145},
                            {"x0":-60.389,"y0":60.799,"x1":27.020,"y1":-80.867,"d":15317.378434391136}]}"#.chars();

        let sum = parse_input(&mut input_iterator);
        assert_eq!(sum, 30923.86682693228);
    }
}
