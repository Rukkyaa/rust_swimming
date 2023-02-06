/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   backup.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/06 10:22:54 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/06 16:31:05 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	parse(a: &[u8]) -> Vec<u8> {
	let mut	v = Vec::new();

	for i in 0..a.len() {
		if !a[i].is_ascii_digit() {panic!("Non numeric number in the string")}
		else {v.push(a[i]);}
	}
	v.reverse();
	v
}

trait Recuperable<T> {
    fn recup(self: &Self, i: usize) -> Option<&T>;
}

impl<T> Recuperable<T> for Vec<T> {
    fn recup(self: &Self, i: usize) -> Option<&T> {
        if i >= self.len() {
            None
        } else {
            Some(&self[i])
        }
    }
}

fn trim_z(vec: Vec<u8>) -> Vec<u8> {
	let mut res = Vec::new();
	let mut i = 0;
	while i < vec.len() && vec[i] == b'0' { i += 1}
	while i < vec.len() {
		res.push(vec[i]);
		i += 1;
	}
	res
}

fn get_num(a: Option<&u8>) -> u8 {
	match a {
		Some(n) => *n,
		None => b'0',
	}
}

fn add(a: Option<&u8>, b: Option<&u8>, ret: &mut u8) -> u8 {
	let a = get_num(a);
	let b = get_num(b);
	let mut res = a + b - 48 + *ret;
	
	if res > b'9' {
		*ret = 1;
		res -= 10
	}
	else { *ret = 0 }
	res
}

fn	big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	let	va = parse(a);
	let	vb = parse(b);
	let mut res = Vec::new();
	let mut ret = 0;
	let mut i = 0;

	while i < va.len() || i < vb.len() {
		res.push(add(va.recup(i), vb.recup(i), &mut ret));
		i += 1
	}
	if ret == 1 { res.push(b'1') }
	res.reverse();
	trim_z(res)
}

fn main() {
	let va = big_add(b"000000099", b"00999");

	for n in va {
		print!("{}", n - 48);
	}
	print!("\n");
}
