/*
 * @Author: 罗华胜 luohuasheng0225@163.com
 * @Date: 2024-07-06 14:05:42
 * @LastEditors: 罗华胜 luohuasheng0225@163.com
 * @LastEditTime: 2024-07-06 14:26:26
 * @FilePath: /exercises/primitive_types/primitive_types6.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.



#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
