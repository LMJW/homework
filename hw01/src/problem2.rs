pub type Matrix = Vec<Vec<f32>>;

pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let r = mat1.len();
    let c = if r > 0 { mat1[0].len() } else { 0 };

    assert_eq!(c, mat2.len());
    let rr = if c > 0 { mat2[0].len() } else { 0 };

    let mut ret = vec![vec![0.; rr]; r];

    for i in 0..r {
        for j in 0..rr {
            let mut tmp = 0.;
            for e in 0..c {
                tmp += mat1[i][e] * mat2[e][j];
            }
            ret[i][j] = tmp;
        }
    }
    ret
}
