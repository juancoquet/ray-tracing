

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    let projection = normal * dot(&incident, &normal);
    let bb = projection * 2.0;
    incident - &bb
}
