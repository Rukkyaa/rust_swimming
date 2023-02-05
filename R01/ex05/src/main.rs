/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: rukkyaa <rukkyaa@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 09:53:00 by rukkyaa           #+#    #+#             */
/*   Updated: 2023/02/05 16:49:38 by rukkyaa          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn deduplicate(list: &mut Vec<i32>) {
    let mut i = 0;
    while i < list.len() {
        let mut j = i + 1;
        while j < list.len() {
            if list[j] == list[i] {
                list.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

fn main() {
	let mut v = vec![1, 2, 2, 3, 2, 4, 3, 2, 2];
	deduplicate(&mut v);
	assert_eq!(v, [1, 2, 3, 4]);
    // println!("Hello, world!");
}
