use std::fmt::format;

#[derive(Debug)]
pub struct MathObject {
    pub data: Vec<u32>,
    pub nb_datum: usize,

    pub min: u32,
    pub max: u32,

    pub average: f64,
    pub variance: f64,
    pub standard_derivation: f64,
}

impl std::fmt::Display for MathObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Data : {:?};\n\
            Minumum = {}; Maximum = {};\n\
            Average = {};\n\
            Standard deviation = {}\n\
            ",
            self.data,
            self.min,
            self.max,
            self.average,
            self.standard_derivation,
        )
    }
}

impl MathObject {
    pub fn new(values : Vec<u32>) -> Result<MathObject, String> {
        let nb_values = values.len();
        if nb_values == 0 {
            return Err("0 values given".to_string());
        }

        let vf64 = values.iter().map(
            |v| { f64::from(*v) }
        ).collect::<Vec<f64>>();

        let avg = {
            let mut avg: f64 = 0f64;
            for v in &vf64 {
                avg += *v;
            }
            avg /= nb_values as f64;
            avg
        };

        let variance = {
            let mut tot: f64 = 0.0;
            for v in vf64.iter() {
                tot = tot + ((v - avg) * (v - avg));
            }

            tot / (values.len() as f64 -1.0)
        };

        let (min, max) = {
            let mut min = values[0];
            let mut max = values[0];
            for v in values.clone() {
                if v.clone() < min {
                    min = v;
                    continue;
                }

                if max < v.clone() {
                    max = v;
                    continue;
                }
            }
            (min, max)
        };

        Ok(MathObject {
            data: values.clone(),
            nb_datum: nb_values,
            min,
            max,
            average: avg,
            variance,
            standard_derivation: f64::sqrt(variance),
        })
    }

    pub fn to_table(&self, args : Vec<String>) -> ((Vec<String>, Vec<String>), (Vec<String>, Vec<String>)) {

        /* Data */

        let mut data_vectors = (vec!["Arguments".to_string()], vec![format!("{args:?}")]);
        for (duration, iteration) in self.data.iter().zip(1..self.data.len()+1) {
            data_vectors.0.push(format!("Iteration n°{iteration}"));
            data_vectors.1.push(format!("{duration} ms"));
        }

        /* Processed data */

        let mut mathobject_vector = (vec![], vec![]);

        mathobject_vector.0.push("Arguments".to_string());
        mathobject_vector.0.push("Minimum".to_string());
        mathobject_vector.0.push("Maximum".to_string());
        mathobject_vector.0.push("Average".to_string());
        mathobject_vector.0.push("Standard derivation".to_string());

        mathobject_vector.1.push(format!("{args:?}"));
        mathobject_vector.1.push(self.min.to_string());
        mathobject_vector.1.push(self.max.to_string());
        mathobject_vector.1.push(self.average.to_string());
        mathobject_vector.1.push(self.standard_derivation.to_string());

        (data_vectors, mathobject_vector)
    }
}