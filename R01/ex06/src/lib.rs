/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/06 10:22:54 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/06 18:28:20 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

trait	Recuperable<T> {
	fn	recup(self: &Self, i: usize) -> Option<&T>;
}

impl<T>	Recuperable<T> for Vec<T> {
	fn	recup(self: &Self, i: usize) -> Option<&T> {
		if i >= self.len() {
			None
		} else {
			Some(&self[i])
		}
	}
}

fn	parse(a: &[u8]) -> Vec<u8> {
	let mut	v = Vec::new();

	if a.is_empty(){panic!("The array is empty !")}
	for i in 0..a.len() {
		if !a[i].is_ascii_digit() {panic!("Non numeric number in the string")}
		else {v.push(a[i]);}
	}
	v.reverse();
	v
}

fn	get_num(a: Option<&u8>) -> u8 {
	match a {
		Some(x) => *x,
		_ => b'0'
	}
}

fn	add(a: Option<&u8>, b: Option<&u8>, ret: &mut u8) -> u8 {
	let x = get_num(a);
	let y = get_num(b);
	let mut	res = x + y - 48 + *ret;

	if res > b'9' {
		*ret = 1;
		res -= 10;
	} else {
		*ret = 0;
	}
	res
}

fn	trim_zero(vec: Vec<u8>) -> Vec<u8> {
	let mut	trimmed_vec = Vec::new();
	let mut i = 0;

	while i < vec.len()-1 && vec[i] == b'0' { i += 1}
	while i < vec.len() {
		trimmed_vec.push(vec[i]);
		i += 1;
	}

	trimmed_vec
}

fn	big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	let	va = parse(a);
	let	vb = parse(b);
	let	mut res = Vec::new();
	let mut	i = 0;
	let mut	ret = 0;

	while i < va.len() || i < vb.len() {
		res.push(add(va.recup(i), vb.recup(i), &mut ret));
		i += 1;
	}
	if ret == 1 { res.push(b'1') }
	res.reverse();
	trim_zero(res)
}

#[cfg(test)]
mod	test {
	use crate:: big_add;
	
	#[test]
	fn	easy_test() {
		assert!(big_add(b"2", b"4") == b"6");
		assert!(big_add(b"10", b"8") == b"18");
		assert!(big_add(b"100", b"42") == b"142");
	}

	#[test]
	fn	basic_test() {
		assert!(big_add(b"120", b"22") == b"142");
		assert!(big_add(b"83", b"16") == b"99");
		assert!(big_add(b"23", b"32") == b"55");
	}

	#[test]
	fn	zero_test() {
		assert!(big_add(b"42", b"0") == b"42");
		assert!(big_add(b"0", b"42") == b"42");
		assert!(big_add(b"0", b"0") == b"0");
		assert!(big_add(b"0010", b"0200") == b"210");
		assert!(big_add(b"007", b"0") == b"7");
		assert!(big_add(b"00700", b"00000000") == b"700");
	}

	#[test]
	fn	carry_test() {
		assert!(big_add(b"9", b"2") == b"11");
		assert!(big_add(b"99", b"1") == b"100");
		assert!(big_add(b"3284", b"39275") == b"42559");
	}

	#[test]
	fn	big_test() {
		assert!(big_add(b"99999999999999999999999999999999999999999999999999", b"1") == b"100000000000000000000000000000000000000000000000000");
		assert!(big_add(b"9823590280573086239", b"4245958903612337415") == b"14069549184185423654");
		assert!(big_add(b"3627586591049095041", b"4723665748976621977") == b"8351252340025717018");
		assert!(big_add(b"1366252263159600498", b"7289523504808362761") == b"8655775767967963259");
		assert!(big_add(b"6342405623940402827", b"8723378674752413191") == b"15065784298692816018");
		assert!(big_add(b"5315858127012569344", b"5426297946789808780") == b"10742156073802378124");
		assert!(big_add(b"6817757680208716792", b"4488434026902281907") == b"11306191707110998699");
	}

	#[test]
	#[should_panic]
	fn	empty_first_test() {
		big_add(b"", b"2");
	}

	#[test]
	#[should_panic]
	fn	empty_second_test() {
		big_add(b"1", b"");
	}

	#[test]
	#[should_panic]
	fn	empty_both_test() {
		big_add(b"", b"");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_1_test() {
		big_add(b"10s0", b"42");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_2_test() {
		big_add(b"42", b"10s0");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_3_test() {
		big_add(b"10s0", b"10s0");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_4_test() {
		big_add(b"z", b"z");
	}
}
