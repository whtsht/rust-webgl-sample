pub struct Matrix {
    value: [f32; 16],
}

#[allow(dead_code)]
impl Matrix {

    //--create new matrix--
    //  <return> Matrix
    pub fn new() -> Self {
        Self {
            value: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ] as [f32; 16],
        }
    }

    //--Get matrix value--
    //  <return> [f32; 16]
    pub fn get_value(&self) -> [f32; 16] {
        self.value
    }

    //--Set a value in the matrix
    //  <argument>
    //      m &[f32; 16] : value to set
    pub fn set_value(&mut self, m: &[f32; 16]) -> &mut Self {
        self.value = *m;
        self
    }

    //--Set the identity matrix in the matrix
    pub fn set_identity(&mut self) -> &mut Self {
        self.value = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ];
        self
    }

    //--Set other matrix value in the matrix
    //  <argument>
    //      m &Matrix : other matrix to set
    pub fn substitution(&mut self, m: &Matrix) -> &mut Self {
        self.value = m.value;
        self
    }

    //--Multiply other matrix
    //  <argument>
    //      m &Matrix : other matrix to multiply
    //  <note>
    //      Column-major order!
    pub fn multiply(&mut self, m: &Matrix) -> &mut Self {
        let mut dest: [f32; 16] = [0.; 16];

        dest[0] = m.value[0] * self.value[0]
            + m.value[1] * self.value[4]
            + m.value[2] * self.value[8]
            + m.value[3] * self.value[12];
        dest[1] = m.value[0] * self.value[1]
            + m.value[1] * self.value[5]
            + m.value[2] * self.value[9]
            + m.value[3] * self.value[13];
        dest[2] = m.value[0] * self.value[2]
            + m.value[1] * self.value[6]
            + m.value[2] * self.value[10]
            + m.value[3] * self.value[14];
        dest[3] = m.value[0] * self.value[3]
            + m.value[1] * self.value[7]
            + m.value[2] * self.value[11]
            + m.value[3] * self.value[15];

        dest[4] = m.value[4] * self.value[0]
            + m.value[5] * self.value[4]
            + m.value[6] * self.value[8]
            + m.value[7] * self.value[12];
        dest[5] = m.value[4] * self.value[1]
            + m.value[5] * self.value[5]
            + m.value[6] * self.value[9]
            + m.value[7] * self.value[13];
        dest[6] = m.value[4] * self.value[2]
            + m.value[5] * self.value[6]
            + m.value[6] * self.value[10]
            + m.value[7] * self.value[14];
        dest[7] = m.value[4] * self.value[3]
            + m.value[5] * self.value[7]
            + m.value[6] * self.value[11]
            + m.value[7] * self.value[15];

        dest[8] = m.value[8] * self.value[0]
            + m.value[9] * self.value[4]
            + m.value[10] * self.value[8]
            + m.value[11] * self.value[12];
        dest[9] = m.value[8] * self.value[1]
            + m.value[9] * self.value[5]
            + m.value[10] * self.value[9]
            + m.value[11] * self.value[13];
        dest[10] = m.value[8] * self.value[2]
            + m.value[9] * self.value[6]
            + m.value[10] * self.value[10]
            + m.value[11] * self.value[14];
        dest[11] = m.value[8] * self.value[3]
            + m.value[9] * self.value[7]
            + m.value[10] * self.value[11]
            + m.value[11] * self.value[15];

        dest[12] = m.value[12] * self.value[0]
            + m.value[13] * self.value[4]
            + m.value[14] * self.value[8]
            + m.value[15] * self.value[12];
        dest[13] = m.value[12] * self.value[1]
            + m.value[13] * self.value[5]
            + m.value[14] * self.value[9]
            + m.value[15] * self.value[13];
        dest[14] = m.value[12] * self.value[2]
            + m.value[13] * self.value[6]
            + m.value[14] * self.value[10]
            + m.value[15] * self.value[14];
        dest[15] = m.value[12] * self.value[3]
            + m.value[13] * self.value[7]
            + m.value[14] * self.value[11]
            + m.value[15] * self.value[15];

        self.value = dest;
        self
    }


    //--Transpose the matrix--
    pub fn transpose(&mut self) -> &mut Self {
        self.value = [
            self.value[0],
            self.value[4],
            self.value[8],
            self.value[12],
            self.value[1],
            self.value[5],
            self.value[9],
            self.value[13],
            self.value[2],
            self.value[6],
            self.value[10],
            self.value[14],
            self.value[3],
            self.value[7],
            self.value[11],
            self.value[15],
        ];

        self
    }

    //--Create translation matrix and multiply it--
    pub fn translation(&mut self, v: &[f32; 3]) -> &mut Self {
        let translation_mat = Matrix {
            value: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., v[0], v[1], v[2], 1.,
            ],
        };

        self.multiply(&translation_mat);
        self
    }

    //--Create scaling matrix and multiply it--
    pub fn scaling(&mut self, v: &[f32; 3]) -> &mut Self {
        let scaling_mat = Matrix {
            value: [
                v[0], 0., 0., 0., 0., v[1], 0., 0., 0., 0., v[2], 0., 0., 0., 0., 1.,
            ],
        };

        self.multiply(&scaling_mat);
        self
    }

    pub fn rotate_around_y(&mut self, rad: f32) -> &mut Self {
        let r_c = rad.cos();
        let r_s = rad.sin();
        let y_mut = Matrix {
            value: [
                r_c, 0., r_s, 0., 0., 1., 0., 0., -r_s, 0., r_c, 0., 0., 0., 0., 1.,
            ],
        };

        self.multiply(&y_mut);
        self
    }

    pub fn rotate_around_x(&mut self, rad: f32) -> &mut Self {
        let r_c = rad.cos();
        let r_s = rad.sin();
        let x_mut = Matrix {
            value: [
                r_c, 0., r_s, 0., 0., 1., 0., 0., -r_s, 0., r_c, 0., 0., 0., 0., 1.,
            ],
        };
        self.multiply(&x_mut);
        self
    }

    pub fn rotate_around_z(&mut self, rad: f32) -> &mut Self {
        let r_c = rad.cos();
        let r_s = rad.sin();
        let z_mut = Matrix {
            value: [
                r_c, -r_s, 0., 0., r_s, r_c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        };
        self.multiply(&z_mut);
        self
    }

    //--Create look_at matrix--
    pub fn look_at(&mut self, from: &[f32; 3], to: &[f32; 3], up: &[f32; 3]) -> &mut Self {
        let mut l: f32;
        let mut x: [f32; 3] = [0.; 3];
        let mut y: [f32; 3] = [0.; 3];
        let mut z: [f32; 3] = [0.; 3];

        z[0] = from[0] - to[0];
        z[1] = from[1] - to[1];
        z[2] = from[2] - to[2];
        l = (z[0] * z[0] + z[1] * z[1] + z[2] * z[2]).sqrt().recip();
        z[0] *= l;
        z[1] *= l;
        z[2] *= l;

        x[0] = up[1] * z[2] - up[2] * z[1];
        x[1] = up[2] * z[0] - up[0] * z[2];
        x[2] = up[0] * z[1] - up[1] * z[0];
        l = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt().recip();
        x[0] *= l;
        x[1] *= l;
        x[2] *= l;

        y[0] = z[1] * x[2] - z[2] * x[1];
        y[1] = z[2] * x[0] - z[0] * x[2];
        y[2] = z[0] * x[1] - z[1] * x[0];
        l = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt().recip();
        y[0] *= l;
        y[1] *= l;
        y[2] *= l;

        let d_12: f32 = -(x[0] * from[0] + x[1] * from[1] + x[2] * from[2]);
        let d_13: f32 = -(y[0] * from[0] + y[1] * from[1] + y[2] * from[2]);
        let d_14: f32 = -(z[0] * from[0] + z[1] * from[1] + z[2] * from[2]);
        self.value = [
            x[0], y[0], z[0], 0.,
            x[1], y[1], z[1], 0.,
            x[2], y[2], z[2], 0.,
            d_12, d_13, d_14, 1.,
        ];
        self
    }

    //--Create perspective projections matrix--
    //  <argument>
    //      aspect f32  ratio parameter is the width divided by the height
    //      fovy   f32  field of view y-axis
    //      near   f32  near clipping plane
    //      far    f32  far clipping plane
    //  <note>
    //      Right-Handed Coordinate System!
    pub fn perspective(&mut self, aspect: f32, fovy: f32, near: f32, far: f32) -> &mut Self {
        let mut dest: [f32; 16] = [0.; 16];
        let t: f32 = (fovy / 2.).tan();
        let d: f32 = far - near;
        dest[0] = 1. / (t * aspect);
        dest[5] = 1. / t;
        dest[10] = - far / d;
        dest[11] = -1.;
        dest[14] = - far * near / d;

        self.value = dest;
        self
    }

    //--inverse the matrix
    pub fn inverse(&mut self) -> Result<&mut Self, i8> {
        const SIZE: usize = 4;
        let mut inv = Matrix::identity();
        let mut buf: f32;
        let mut a = self.value;

        for i in 0..SIZE {
            if a[i * SIZE + i] == 0. {
                return Err(-1);
            }
            buf = 1. / a[i * SIZE + i];
            for j in 0..SIZE {
                a[i * SIZE + j] *= buf;
                inv[i * SIZE + j] *= buf;
            }
            for j in 0..SIZE {
                if i != j {
                    buf = a[j * SIZE + i];
                    for k in 0..SIZE {
                        a[j * SIZE + k] -= a[i * SIZE + k] * buf;
                        inv[j * SIZE + k] -= inv[i * SIZE + k] * buf;
                    }
                }
            }
        }

        self.value = inv;
        Ok(self)
    }

    fn identity() -> [f32; 16] {
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ] as [f32; 16]
    }
}
