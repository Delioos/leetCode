let arr1: number[] = [2,3,1,3,2,4,6,7,9,2,19];
let arr2: number[] = [2,1,4,3,9,6];

function relativeSortArray(arr1: number[], arr2: number[]): number[] {
    let nums: number[] = [];

    for (let n of arr2) {
        nums.push(...arr1.filter((x: number) => x === n));
    }

    let temp: number[] = [];

    temp = arr1.filter((x: number) => !arr2.includes(x));
    temp.sort((a: number, b: number) => a - b);
    nums = nums.concat(temp);

    return nums;
}