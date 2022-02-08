use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Matrix<const N: usize>([[u64;N];N]);

impl<const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;
    
    fn mul(self, Matrix(rhs): Self) -> Self {
        let Matrix(lhs) = self;
        let mut result = [[0;N];N];
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[i][k] += lhs[i][j] * rhs[j][k];
                }
            }
        }
        Matrix(result)
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, rhs: &Matrix<N>) -> bool {
        self.0 == rhs.0
    }
}

const TTT: Matrix<3> = Matrix(
    [[1,2,3],
     [4,5,6],
     [7,8,9]]
);
                      
const ID: Matrix<3> = Matrix(
    [[1,0,0],
     [0,1,0],
     [0,0,1]]
);

const ID2: Matrix<2> = Matrix(
    [[1,0],
     [0,1]]
);

const MAGIC: Matrix<3> = Matrix(
    [[6,1,8],
     [7,5,3],
     [2,9,4]]
);

fn main() {
    assert_eq!(TTT * ID, TTT);
    assert_eq!(ID * TTT, TTT);
    assert!(TTT * MAGIC != MAGIC * TTT);
    assert_eq!(ID2 * ID2, ID2);
    println!("{:?}", TTT * TTT);
}
