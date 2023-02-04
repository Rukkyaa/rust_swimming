/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: rukkyaa <rukkyaa@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/04 14:18:58 by rukkyaa           #+#    #+#             */
/*   Updated: 2023/02/04 14:56:30 by rukkyaa          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]
const fn	color_name(color: &[u8; 3]) -> &'static str
{
	match color {
		[0, 0, 0] => "pure black",
		[255, 255, 255] => "pure white",
		[255, 0, 0] => "pure red",
		[0, 255, 0] => "pure green",
		[0, 0, 255] => "pure blue",
		[128, 128, 128] => "perfect grey",
		[0..=30, 0..=30, 0..=30] => "almost black",
		[128..=255, 0..=127, 0..=127] => "redish",
		[0..=127, 128..=255, 0..=127] => "greenish",
		[0..=127, 0..=127, 128..=255] => "blueish",
		_ => "unknown"
	}
}

#[cfg(test)]
mod	test
{
	use	crate::color_name;

	#[test]
	fn	pure_black_test() {
		assert!(color_name(&[0, 0, 0]) == "pure black", "Pure black test failed");
	}

	#[test]
	fn	pure_white_test() {
		assert!(color_name(&[255, 255, 255]) == "pure white", "Pure white test failed");
	}

	#[test]
	fn	pure_red_test() {
		assert!(color_name(&[255, 0, 0]) == "pure red", "Pure red test failed");
	}

	#[test]
	fn	pure_green_test() {
		assert!(color_name(&[0, 255, 0]) == "pure green", "Pure green test failed");
	}

	#[test]
	fn	pure_blue_test() {
		assert!(color_name(&[0, 0, 255]) == "pure blue", "Pure blue test failed");
	}

	#[test]
	fn	perfect_grey_test() {
		assert!(color_name(&[128, 128, 128]) == "perfect grey", "Perfect grey test failed");
	}

	#[test]
	fn	almost_black_test() {
		assert!(color_name(&[30, 30, 30]) == "almost black", "Almost black test failed");
		assert!(color_name(&[0, 0, 30]) == "almost black", "Almost black test failed");
		assert!(color_name(&[30, 0, 00]) == "almost black", "Almost black test failed");
		assert!(color_name(&[0, 30, 0]) == "almost black", "Almost black test failed");
		assert!(color_name(&[1, 1, 1]) == "almost black", "Almost black test failed");
	}

	#[test]
	fn	redish_test() {
		assert!(color_name(&[128, 127, 127]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 127, 0]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 0, 127]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 1, 1]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 0, 0]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 0, 1]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 1, 0]) == "redish", "Redish test failed");
	}

	#[test]
	fn	greenish_test() {
		assert!(color_name(&[127, 128, 127]) == "greenish", "Greenish test failed");
		assert!(color_name(&[127, 255, 0]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 255, 127]) == "greenish", "Greenish test failed");
		assert!(color_name(&[1, 255, 1]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 128, 0]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 128, 1]) == "greenish", "Greenish test failed");
		assert!(color_name(&[1, 128, 0]) == "greenish", "Greenish test failed");
	}

	#[test]
	fn	blueish_test() {
		assert!(color_name(&[127, 127, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[127, 0, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 127, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[1, 1, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 0, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 1, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[1, 0, 128]) == "blueish", "Blueish test failed");
	}

	#[test]
	fn	unknown_test() {
		assert!(color_name(&[255, 255, 0]) == "unknown", "Unknown test failed");
		assert!(color_name(&[255, 0, 255]) == "unknown", "Unknown test failed");
		assert!(color_name(&[0, 255, 255]) == "unknown", "Unknown test failed");
		assert!(color_name(&[32, 32, 32]) == "unknown", "Unknown test failed");
		assert!(color_name(&[42, 42, 42]) == "unknown", "Unknown test failed");
		assert!(color_name(&[127, 127, 127]) == "unknown", "Unknown test failed");
	}
}
