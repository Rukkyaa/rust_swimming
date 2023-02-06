/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: axlamber <axlamber@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/03 12:02:19 by axlamber          #+#    #+#             */
/*   Updated: 2023/02/03 16:36:34 by axlamber         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ex07::strpcmp;

fn main(){
    let arg = ftkit::ARGS;

    if arg.len() < 3
    {
        println!("Invalid number of arguments");
        return ;
    }
    println!("{}", ["no", "yes"][strpcmp(arg[1].as_bytes(), arg[2].as_bytes()) as usize]);
}

#[cfg(test)]
mod test {
    use crate::strpcmp;

    #[test]
    fn basic_test() {
        assert!(strpcmp(b"salut", b"salut"));
        assert!(strpcmp(b"42", b"42"));
        assert!(strpcmp(b"abc", b"abc"));
        assert!(!strpcmp(b"abc", b"abd"));
        assert!(!strpcmp(b"abc", b"bbc"));
    }

    #[test]
    fn wildcard() {
        assert!(strpcmp(b"abc", b"*"));
        assert!(strpcmp(b"abc", b"a*"));
        assert!(strpcmp(b"abc", b"ab*"));
        assert!(strpcmp(b"abc", b"******"));
        assert!(strpcmp(b"abc", b"a*****c"));
        assert!(strpcmp(b"abc", b"a***b**c"));
        assert!(!strpcmp(b"abc", b"a*abc"));
        assert!(!strpcmp(b"abc", b"a*bb*cc"));
        assert!(strpcmp(b"ab000cd", b"ab*cd"));
        assert!(strpcmp(b"abcd", b"ab*cd"));
    }

    #[test]
    fn empty() {
        assert!(strpcmp(b"", b""));
        assert!(strpcmp(b"", b"*"));
        assert!(strpcmp(b"", b"**********"));
        assert!(!strpcmp(b"", b"a*"));
        assert!(!strpcmp(b"", b"*a*"));
        assert!(!strpcmp(b"", b"*a"));
    }

    #[test]
    fn ultimate() {
        assert!(strpcmp(b"abbcdefcdeeeffef", b"ab*cd*ef"));
    }
}