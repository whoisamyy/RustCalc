use std::fmt::Display;

pub struct BinaryTree<T> {
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,

    value: T
}

impl<T> BinaryTree<T> {
    fn new(value: T) -> BinaryTree<T> {
        BinaryTree {
            left: None,
            right: None,
            value,
        }
    }
}

impl Display for BinaryTree<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.value.parse::<i32>();
        if val.is_ok() {
            return write!(f, "{}", self.value);
        }

        let op = self.value.as_str();
        let left = &self.left.as_ref();
        let right = &self.right.as_ref();
        match op {
            "*" => return write!(f, "*({}, {})", left.unwrap().to_string(), right.unwrap().to_string()),
            "/" => return write!(f, "/({}, {})", left.unwrap().to_string(), right.unwrap().to_string()),
            "+" => return write!(f, "+({}, {})", left.unwrap().to_string(), right.unwrap().to_string()),
            "-" => return write!(f, "-({}, {})", left.unwrap().to_string(), right.unwrap().to_string()),
            _ => {}
        }

        panic!("err")
    }
}

fn main() {
    let expr = parse_expression("1 + 2 * 3 - 6".to_string()).unwrap();

    print!("{}\n", expr);
    print!("{}", solve(&*expr));
}

fn solve(expr_tree: &BinaryTree<String>) -> i32 {
    let val = expr_tree.value.parse::<i32>();
    if val.is_ok() {
        return val.unwrap();
    }

    let op = expr_tree.value.as_str();
    let left = &expr_tree.left.as_ref();
    let right = &expr_tree.right.as_ref();
    match op {
        "*" => return solve(left.unwrap().as_ref()) * solve(right.unwrap().as_ref()),
        "/" => return solve(left.unwrap().as_ref()) / solve(right.unwrap().as_ref()),
        "+" => return solve(left.unwrap().as_ref()) + solve(right.unwrap().as_ref()),
        "-" => return solve(left.unwrap().as_ref()) - solve(right.unwrap().as_ref()),
        _ => {}
    }
    
    panic!("err")
}

fn parse_expression(mut string: String) -> Option<Box<BinaryTree<String>>> {
    string = string.trim().to_string();
    let parse_res = string.parse::<i32>();
    if parse_res.is_ok() {
        return Some(Box::new(BinaryTree::new(string)));
    }

    let mut split = string.split_once('+');
    if split.is_some() {
        let (left, right) = split.unwrap();

        return Some(Box::new(BinaryTree {
            left: parse_expression(left.to_string()),
            right: parse_expression(right.to_string()),
            value: "+".to_string()
        }))
    }

    split = string.split_once('-');
    if split.is_some() {
        let (left, right) = split.unwrap();

        return Some(Box::new(BinaryTree {
            left: parse_expression(left.to_string()),
            right: parse_expression(right.to_string()),
            value: "-".to_string()
        }))
    }

    split = string.split_once('*');
    if split.is_some() {
        let (left, right) = split.unwrap();

        return Some(Box::new(BinaryTree {
            left: parse_expression(left.to_string()),
            right: parse_expression(right.to_string()),
            value: "*".to_string()
        }))
    }

    split = string.split_once('/');
    if split.is_some() {
        let (left, right) = split.unwrap();

        return Some(Box::new(BinaryTree {
            left: parse_expression(left.to_string()),
            right: parse_expression(right.to_string()),
            value: "/".to_string()
        }))
    }

    None
}
