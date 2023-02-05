/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: rukkyaa <rukkyaa@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 08:49:35 by rukkyaa           #+#    #+#             */
/*   Updated: 2023/02/05 09:42:38 by rukkyaa          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	can_fit_in(a: [u32; 2], b: [u32; 2]) -> bool{
	if a[0] == b[0] && a[1] == b[1] {return false}
	a[0] <= b[0] && a[1] <= b[1]
}

fn	is_sort(boxes: &mut [[u32; 2]]) -> bool {
	for i in 0..boxes.len() - 1 {
		if boxes[i][0] < boxes[i+1][0] || boxes[i][1] < boxes[i+1][1] {return false}
	}
	true
}

fn	sort_boxes(boxes: &mut [[u32; 2]]) {
	let mut	i = 0;

	if boxes.is_empty() {panic!("The box is empty")}
	while i < boxes.len() - 1 {
		if can_fit_in(boxes[i], boxes[i + 1]) {
			boxes.swap(i, i+1);
			i = 0
		}
		else {i += 1}
	}
	if !is_sort(boxes) {panic!("The boxes cant be sort")}
}

#[cfg(test)]
mod	sort_boxes {
	use crate::sort_boxes;

	#[test]
	fn	exemple_test() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	fn	already_sort_test() {
		let mut	boxes = [[5, 5], [5, 4], [4, 4], [3, 4], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 5], [5, 4], [4, 4], [3, 4], [3, 3]]);
	}

	#[test]
	fn	reverse_test() {
		let mut boxes = [[1, 0], [3, 3], [3, 3], [4, 3], [5, 7]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	fn	duplicate_test() {
		let mut	boxes = [[1, 1], [2, 3], [1, 1], [2, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[2, 3], [2, 3], [1, 1], [1, 1]]);
	}

	#[test]
	fn	single_box_test() {
		let mut	boxes = [[42, 42]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[42, 42]]);
	}

	#[test]
	#[should_panic(expected = "The box is empty")]
	fn	empty_test() {
		sort_boxes(&mut []);
	}

	#[test]
	#[should_panic(expected = "The boxes cant be sort")]
	fn	impossible_test() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3], [6, 6]];
		sort_boxes(&mut boxes);
	}
}
