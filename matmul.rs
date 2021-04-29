use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Matrix3([[u64;3];3]);

impl Mul for Matrix3 {
    type Output = Matrix3;
    
    fn mul(self, Matrix3(rhs): Self) -> Self {
        let Matrix3(lhs) = self;
        let mut result = [[0;3];3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][k] += lhs[i][j] * rhs[j][k];
                }
            }
        }
        Matrix3(result)
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, rhs: &Matrix3) -> bool {
        self.0 == rhs.0
    }
}

const TTT: Matrix3 = Matrix3(
    [[1,2,3],
     [4,5,6],
     [7,8,9]]
);
                      
const ID: Matrix3 = Matrix3(
    [[1,0,0],
     [0,1,0],
     [0,0,1]]
);

const MAGIC: Matrix3 = Matrix3(
    [[6,1,8],
     [7,5,3],
     [2,9,4]]
);

fn main() {
    assert_eq!(TTT * ID, TTT);
    assert_eq!(ID * TTT, TTT);
    assert!(TTT * MAGIC != MAGIC * TTT);
    println!("{:?}", TTT * TTT);
}
