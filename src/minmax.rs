use std::ops::{Add,Sub};
use log::{ info, error, debug, /*warn,*/ trace };

use std::fmt;
use crate::minmax::MinMax::{Value,NA};

#[derive(Debug,Clone,Copy,PartialOrd,Ord,PartialEq,Eq)]
pub enum MinMax<T> {
    Min,
    Value(T),
    Max,
    NA,
}

// Implement `Display` for `MinMax`.
impl<T: fmt::Display> fmt::Display for MinMax<T>

{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinMax::Min => f.pad(&format!("Min")),
            MinMax::Max => f.pad(&format!("Max")),
            MinMax::NA =>  f.pad(&format!("NA")),
            MinMax::Value(ref x) =>  f.pad(&format!("{}", x))
        }
    }
}

impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>> Add for MinMax<T> {
    type Output = MinMax<T>;

    fn add(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(op2)) | (MinMax::NA, Value(op2)) => Value(op2),
            (MinMax::Max,_) | (_, MinMax::Max) => MinMax::Max,
            (Value(op1), Value(op2)) => Value(op1+op2),
        }
    }
}

impl<T: std::ops::Sub + std::cmp::PartialEq + Sub<Output = T>> Sub for MinMax<T> {
    type Output = MinMax<T>;

    fn sub(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(_)) => MinMax::Min,
            (MinMax::NA, Value(_)) => MinMax::Min,
            (MinMax::Max,_) => MinMax::Max,
            (_, MinMax::Max) => MinMax::Min,
            (Value(op1), Value(op2)) => Value(op1-op2),
        }
    }
}

