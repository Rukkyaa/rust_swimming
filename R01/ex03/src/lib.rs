/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: rukkyaa <rukkyaa@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/04 15:28:22 by rukkyaa           #+#    #+#             */
/*   Updated: 2023/02/04 17:55:34 by rukkyaa          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	is_valid_number(number: u32, needle: &[u32]) -> bool {
	for elem in needle {
		if number == *elem {return true;}
	}
	false
}

fn	full_needle_in_slice(haystack: &[u32], needle: &[u32]) -> bool {
	for elem in needle {
		if !haystack.contains(elem) {return false;}
	}
	true
}

fn	get_slice_len(haystack: &[u32], needle: &[u32]) -> usize {
	let mut	i = 0;

	if !full_needle_in_slice(haystack, needle) {return 0;}
	for e in 0..haystack.len() {
		if !is_valid_number(haystack[e], needle) {
			if !full_needle_in_slice(&haystack[..=e], needle) {return 0;}
			return i;
		}
		i += 1;
	}
	i
}

fn	largest_group<'a>(haystack: &'a [u32], needle: &'a [u32]) -> &'a [u32] {
	let mut	slice : &[u32] = &[];

	for i in 0..haystack.len() {
		if get_slice_len(&haystack[i..], needle) > slice.len() {
			slice = &haystack[i..i+get_slice_len(&haystack[i..], needle)];
		}
	}
	slice
}

#[cfg(test)]
mod	test {
	use crate::largest_group;

	#[test]
	fn	basic_test() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3, 5]), &[3, 5, 5]);
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
		assert_eq!(largest_group(&[5], &[5, 5]), &[5]);
		assert_eq!(largest_group(&[1, 2, 3, 2, 1], &[1, 2, 3]), &[1, 2, 3, 2, 1]);
	}

	#[test]
	fn	empty_test(){
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
		assert_eq!(largest_group(&[], &[5, 1, 2]), &[]);
		assert_eq!(largest_group(&[], &[]), &[]);
		assert_eq!(largest_group(&[1, 3, 2, 2, 1, 3, 4], &[5]), &[]);
	}

	// fn test_lifetimes() {
	// 	let haystack = [1, 2, 3, 2, 1];
	// 	let result;
	
	// 	{
	// 		let needle = [2, 3];
	// 		result = largest_group(&haystack, &needle);
	// 	}

	// 	assert_eq!(result, &[2, 3, 2]);
	// }
}
