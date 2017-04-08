use cf::perform_bool_op::{AfBoolOp, InfoBoolOp};
use bo::*;
use cf::{Af, create};
use cf::triangulate::*;
use cf::intersect::*;
use qm::*;
use std::collections::{HashMap, BTreeSet};

#[derive(Default)]
pub struct RafSimpleBoolOp
<QM : QualityMetric = UselessQM,
    RafMxM : AfMxM = RafSimpleMxM,
    RafTr : AfTriangulate = RafIncrementalTriangulation>
{
    pub qm : QM,
    pub raf_mxm : RafMxM,
    pub raf_tr : RafTr
}

impl
<QM : QualityMetric,
    RafMxM : AfMxM,
    RafTr : AfTriangulate>
Af for RafSimpleBoolOp<QM, RafMxM, RafTr> {}


impl
<QM : QualityMetric,
    RafMxM : AfMxM,
    RafTr : AfTriangulate>
AfBoolOp for RafSimpleBoolOp<QM, RafMxM, RafTr> {
    fn split(&mut self, ma : &Mesh, mb : &Mesh) -> (Vec<Triangle>, Vec<Triangle>, Vec<Triangle>, Vec<Triangle>) {
        let mut raf_simple_mxm : RafMxM = create();
        let ires = raf_simple_mxm.intersect(ma, mb);
        let raf_tr : RafTr = create();

        let mut a_map : HashMap<usize, TrD> = HashMap::new();
        let mut b_map : HashMap<usize, TrD> = HashMap::new();

        // нужно сгенерить структуры описатели треугольников!
        for ((index_a, index_b), res) in ires.map {
            let mut s : Segment = res.0.unwrap();

            if a_map.contains_key(&index_a) {
                let mut t_b : Triangle = mb.get_triangle(index_b);
                // let mut normal = t_b.get_normal();
                let mut normal = mb.get_normal_of_triangle(index_b);

                a_map.get_mut(&index_a).unwrap().i_points.push(
                    (s.org.clone(), s.dest.clone(), normal)
                );
            } else {
                let mut t_a : Triangle = ma.get_triangle(index_a);
                let mut tr_points : Vec<Point> = vec![t_a.p1, t_a.p2, t_a.p3];
                let mut i_points : Vec<(Point, Point, Vector)> = Vec::new();

                let mut t_b : Triangle = mb.get_triangle(index_b);
                // let mut normal = t_b.get_normal();
                let mut normal = mb.get_normal_of_triangle(index_b);
                i_points.push(
                    (s.org.clone(), s.dest.clone(), normal)
                );
                let desc = TrD {
                    tr_points: tr_points,
                    i_points: i_points
                };
                a_map.insert(index_a, desc);
            }

            if b_map.contains_key(&index_b) {
                let mut t_a : Triangle = ma.get_triangle(index_a);
                // let mut normal = t_a.get_normal();
                let mut normal = ma.get_normal_of_triangle(index_a);

                b_map.get_mut(&index_b).unwrap().i_points.push(
                    (s.org, s.dest, normal)
                );
            } else {
                let mut t_b : Triangle = mb.get_triangle(index_b);
                let mut tr_points : Vec<Point> = vec![t_b.p1, t_b.p2, t_b.p3];
                let mut i_points : Vec<(Point, Point, Vector)> = Vec::new();
                let mut t_a : Triangle = ma.get_triangle(index_a);
                // let mut normal = t_a.get_normal();
                let mut normal = ma.get_normal_of_triangle(index_a);

                i_points.push((s.org, s.dest, normal));
                let desc = TrD {
                    tr_points: tr_points,
                    i_points: i_points
                };
                b_map.insert(index_b, desc);
            }
            //print!("{:?} {:?}\n", index_a, index_b);
        }
        let mut raf_tr : RafTr = create();

        //ia - intersection area
        //um - updated mesh
        //в моделе A заменяем все треугольники из зоны пересечения их триангуляцией
        println!("model A");
        let (uma, ia_a) : (Mesh, Vec<(usize, Marker)>) = create_um::<RafTr>(&ma, &a_map);
        //в моделе B заменяем все треугольники из зоны пересечения их триангуляцией
        println!("model B");
        let (umb, ia_b) : (Mesh, Vec<(usize, Marker)>) = create_um::<RafTr>(&mb, &b_map);

        let (inner_part_a, outer_part_a) = separate(&uma, ia_a);
        let (inner_part_b, outer_part_b) = separate(&umb, ia_b);

        /*
        println!("inner_part_a: {:?}", inner_part_a.len());
        println!("outer_part_a: {:?}", outer_part_a.len());
        println!("uma {:?}", uma.triangles.len());
        panic!();
        */
        return (inner_part_a, outer_part_a, inner_part_b, outer_part_b);
    }
    fn union(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (_, outer_part_a, _, outer_part_b) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(outer_part_a);
        mesh.add_triangles(outer_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }

    fn intersect(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (inner_part_a, _, inner_part_b, _) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(inner_part_a);
        mesh.add_triangles(inner_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }

    fn difference(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (_, outer_part_a, inner_part_b, _) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(outer_part_a);
        mesh.add_triangles(inner_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }
}



fn create_um<RafTr : AfTriangulate>(m : &Mesh, map : &HashMap<usize, TrD>) -> (Mesh, Vec<(usize, Marker)>) {
    let mut raf_tr : RafTr = create();
    let mut um : Mesh = Mesh::new();
    let mut ia : Vec<(usize, Marker)> = Vec::new();
    for index in 0 .. m.triangles.len() {
        //println!("triangulation {0}", index);
        if !map.contains_key(&index) {
            um.add_triangle(m.get_triangle(index));
        } else {
            let desc : &TrD = map.get(&index).unwrap();
            let ps : Vec<Point> = desc.extract_points();
            let ts = raf_tr.triangulate(ps);
            for mut t in ts {
                let pair = (um.triangles.len(), classify(&mut t, &desc));
                ia.push(pair);
                um.add_triangle(t);
            }
        }
    }
    return (um, ia);
}


fn classify(t : &mut Triangle, desc : &TrD) -> Marker {
    // цикл по звеньям кривой
    for &(ref p1, ref p2, ref normal) in &desc.i_points {
        // если какое-то звено кривой является стороной треугольника => классифицируем
        // 10 - волшебное число приносящее удачу
        let mut i1 : usize = 10;
        let mut i2 : usize = 10;

        //ищем точки которые лежат на кривой
        for i in 1..4 {
            if p1.eq(t.get(i)) {
                i1 = i;
                continue;
            }
            if p2.eq(t.get(i)) {
                i2 = i;
                continue;
            }
        }

        // Если были найдены две точки на кривой пересечения
        if i1 != 10 && i2 != 10 {
            let j = 6 - i1 - i2;
            let x = (t.get(i1).x + t.get(i2).x)/2.;
            let y = (t.get(i1).y + t.get(i2).y)/2.;
            let z = (t.get(i1).z + t.get(i2).z)/2.;
            let middle = Point::new(x,y,z);

            //если треугольник узкий, то эти две точки могут слабо различаться => много погрешности
            let mut v : Vector = t.get(j) - &middle;

            v.normalize();
            /*
            скалярное произведение нормали и вектора v
            вектор имеет начало в середине стороны, лежащей на линии пересечения,
            а заканчивается в третьей вершине треугольника
            */

            let cp = v.dot_product(&normal);
            // TODO: figure out why sign is incorrect!
            //if cp > 0. {

            if cp > -0.05 && cp < 0.05 {
                println!("Perhaps error!");
                //return Marker::BoundInner;
            }

            if t.get_normal().is_zero() {
                println!("Perhaps error zero normal!");
                //return Marker::BoundInner;
            }

            if cp < 0. {
                //println!("Outer");
                return Marker::BoundOuter;
            } else {
                //println!("Inner");
                return Marker::BoundInner;
            }
        }
    }

    //println!("Bound");
    return Marker::Bound;
}

#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
enum Marker {
    Unclassified,
    Inner,
    Outer,
    Bound,
    BoundInner,
    BoundOuter
}

fn separate(um : &Mesh, mut ia : Vec<(usize, Marker)>) -> (Vec<Triangle>, Vec<Triangle>) {
    let mut inner_part : Vec<Triangle> = Vec::new();
    let mut outer_part : Vec<Triangle> = Vec::new();

    // массив с разметкой вершин
    let mut markers : Vec<Marker> = vec![Marker::Unclassified; um.triangles.len()];

    // заполняем маркеры уже известными
    // изевстны маркеры на границе Bound, BoundInner, BoundOuter
    // Граничные треугольники, помеченные как Bound, имеют лишь одну точку касания с линией пересечения.
    for (i, marker) in ia.clone() {
        markers[i] = marker;
    }

    for &(i, ref marker) in &ia {
        if *marker == Marker::Bound {
            for ni in um.triangles[i].neighbors.clone() {
                let n_tr = &um.triangles[ni];
                if (markers[ni] == Marker::BoundInner || markers[ni] == Marker::BoundOuter)
                    && um.triangles[i].contain_two_points(n_tr) {
                    markers[i] = markers[ni].clone();
                    //marker = markers[ni].clone();
                    //println!("Changed step 1");
                    break;
                }
            }
        }
    }

    // Non-recursive dfs
    let mut nodes_to_visit : Vec<usize> = Vec::new();
    for (i, _) in ia {
        let marker = markers[i].clone();
        // маркер которым будем красить вершины
        let cur_marker : Marker;
        match marker {
            Marker::BoundInner => cur_marker = Marker::Inner,
            Marker::BoundOuter => cur_marker = Marker::Outer,
            _ => panic!("Smth goes wrong!")
        }
        // Добавляем индекс треугольника в список на посещение
        nodes_to_visit.push(i);
        while !nodes_to_visit.is_empty() {
            let cur_index = nodes_to_visit.pop().unwrap();
            //println!("Cur marker {:?}!", marker);
            //перебираю соседей данного треугольника
            // ni - индекс соседа
            for ni in &um.triangles[cur_index].neighbors {
                match markers[*ni].clone() {
                    Marker::BoundInner | Marker::BoundOuter => (),
                    Marker::Inner | Marker::Outer => (),
                    Marker::Unclassified => {
                        markers[*ni] = cur_marker.clone();
                        nodes_to_visit.push(*ni);
                    },
                    Marker::Bound => {
                        if markers[cur_index] != Marker::BoundInner && markers[cur_index] != Marker::BoundOuter {
                            // размечаем Bound реугольники только если текущий треугольник внутренний

                            println!("Changed step 2");
                            markers[*ni] = marker.clone();
                        }
                    },
                    _ => panic!("Something goes wrong")
                }
            }
        }

    }

    //println!("markers {:?}", markers);

    let mut index = 0;
    for marker in markers {
        match marker {
            Marker::BoundInner | Marker::Inner => {
                inner_part.push(um.get_triangle(index));
            },

            Marker::BoundOuter | Marker::Outer => {
                outer_part.push(um.get_triangle(index));
            },

            marker => {
                //inner_part.push(um.get_triangle(index))


                panic!("Incorrect triangle marker! {:?}", marker)
            }
        }
        index += 1;
    }

    return (inner_part, outer_part);
}

/*
fn dfs(
    um : &Mesh,
    inner : &Vec<Triangle>,
    outer : &Vec<Triangle>,
    ia : &BTreeSet<usize>,
    markers : & mut Vec<Markers>,
    i : usize
) {
    for ni in &um.triangles[i].neighbors {
        if !ia.contains(ni) {
            dfs(um, inner, outer, ia, markers, ni);
        }

    }
}
*/

//Triangle Descriptor
struct TrD {
    pub tr_points : Vec<Point>,
    //список сегментов кривой пересечения и нормаль к каждому сегменту
    pub i_points : Vec<(Point, Point, Vector)>
}

impl TrD {
    fn extract_points(&self) -> Vec<Point> {
        let mut v : Vec<Point> = Vec::new();
        for p in &self.tr_points {
            if !v.contains(&p) {
                v.push(p.clone());
            }

        }
        for &(ref p1,ref p2, _) in &self.i_points {
            if !v.contains(&p1) {
                v.push(p1.clone());
            }
            if !v.contains(&p2) {
                v.push(p2.clone());
            }
        }
        return v;
    }
}

#[cfg(test)]
mod tests {
    use cf::perform_bool_op::*;
    use cf::create;
    use bo::*;
    use std::option::Option;
    use std::fs::File;
    //use qm::*;


    fn union_test(input_file_name_a : &str, input_file_name_b : &str, output_file_name : &str) {
        let mut fa = File::open(input_file_name_a).unwrap();
        let mut fb = File::open(input_file_name_b).unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.union(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create(output_file_name).unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }
    }

    fn intersect_test(input_file_name_a : &str, input_file_name_b : &str, output_file_name : &str) {
        //cargo test first_union_test -- --nocapture
        let mut fa = File::open(input_file_name_a).unwrap();
        let mut fb = File::open(input_file_name_b).unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.intersect(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create(output_file_name).unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }
    }

    fn difference_test(input_file_name_a : &str, input_file_name_b : &str, output_file_name : &str) {
        //cargo test first_union_test -- --nocapture
        let mut fa = File::open(input_file_name_a).unwrap();
        let mut fb = File::open(input_file_name_b).unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.difference(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create(output_file_name).unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }
    }

    #[test]
    fn first_union_test() {
        union_test("input_for_tests/cube_in_origin.stl",
                   "input_for_tests/scaled_shifted_cube.stl",
                   "res_of_tests/simple_bool_op/union_test1.stl");
    }

    #[test]
    fn first_intersect_test() {
        intersect_test("input_for_tests/cube_in_origin.stl",
                       "input_for_tests/scaled_shifted_cube.stl",
                       "res_of_tests/simple_bool_op/intersect_test1.stl");
    }

    #[test]
    fn first_difference_test() {
        // TODO исправить нормали!!!
        difference_test("input_for_tests/cube_in_origin.stl",
                        "input_for_tests/scaled_shifted_cube.stl",
                        "res_of_tests/simple_bool_op/difference_test1.stl");
    }


    #[test]
    fn second_union_test() {
        //cargo test first_union_test -- --nocapture
        union_test("input_for_tests/cube_in_origin.stl",
                   "input_for_tests/long_scaled_shifted_cube.stl",
                   "res_of_tests/simple_bool_op/union_test2.stl");
    }

    #[test]
    fn second_intersect_test() {
        intersect_test("input_for_tests/cube_in_origin.stl",
                       "input_for_tests/long_scaled_shifted_cube.stl",
                       "res_of_tests/simple_bool_op/intersect_test2.stl");
    }

    #[test]
    fn second_difference_test() {
        difference_test("input_for_tests/cube_in_origin.stl",
                        "input_for_tests/long_scaled_shifted_cube.stl",
                        "res_of_tests/simple_bool_op/difference_test2.stl");
    }



    #[test]
    fn third_union_test() {
        //cargo test first_union_test -- --nocapture
        union_test("input_for_tests/sphere_in_origin.stl",
                   "input_for_tests/long_scaled_shifted_cube.stl",
                   "res_of_tests/simple_bool_op/union_test3.stl");
    }


    #[test]
    fn third_intersect_test() {
        intersect_test("input_for_tests/sphere_in_origin.stl",
                       "input_for_tests/long_scaled_shifted_cube.stl",
                       "res_of_tests/simple_bool_op/intersect_test3.stl");
    }


    #[test]
    fn third_difference_test() {
        difference_test("input_for_tests/sphere_in_origin.stl",
                        "input_for_tests/long_scaled_shifted_cube.stl",
                        "res_of_tests/simple_bool_op/difference_test3.stl");
    }


    #[test]
    fn fourth_union_test() {
        //cargo test first_union_test -- --nocapture
        union_test("input_for_tests/sphere_in_origin.stl",
                   "input_for_tests/cone_in_origin.stl",
                   "res_of_tests/simple_bool_op/union_test4.stl");
    }

    #[test]
    fn fourth_intersect_test() {
        intersect_test("input_for_tests/sphere_in_origin.stl",
                       "input_for_tests/cone_in_origin.stl",
                       "res_of_tests/simple_bool_op/intersect_test4.stl");
    }


    #[test]
    fn fourth_difference_test() {
        difference_test("input_for_tests/sphere_in_origin.stl",
                        "input_for_tests/cone_in_origin.stl",
                        "res_of_tests/simple_bool_op/difference_test4.stl");
    }

}

