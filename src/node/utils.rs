pub fn name_to_port(name : String)->usize
{
    let mut count = 0;
    let mut by = 1 as usize;
    for i in name.clone().bytes()
    {
        count += (i as usize) * by;
        by += 1;
    }

    count
}