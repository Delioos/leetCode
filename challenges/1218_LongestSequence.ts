function longestSubSequence(arr: number[], diff: number): number {
  let maxSeqLength = 0;
  let tempSeqLength;

  for (let i = 1; i < arr.length; i++) {
    tempSeqLength = 1;

    for (let j = i; j < arr.length; j++) {
      let currDiff = arr[j] - arr[i - 1];
      if (currDiff === diff * tempSeqLength) {
        tempSeqLength++;
      }
    }
    if (tempSeqLength > maxSeqLength) {
      maxSeqLength = tempSeqLength;
    }
  }
    return maxSeqLength;
}
