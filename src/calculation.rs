pub fn calculation(level : usize, move_power : usize, atk : usize, def : usize){
	let d = (level * 2) / 5 + 2;
	let d = d * move_power * atk / def;
	let d = d / 50 + 2;
	
}