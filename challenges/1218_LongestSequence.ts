function longestSubSequence(arr: number[], diff: number): number {
  let maxSeqLength = 0;
  const seqLengths = new Map<number, number>();

  for (const currNum of arr) {
    const prevNum = currNum - diff;
    const prevSeqLength = seqLengths.get(prevNum) || 0; // if prevNum is not in seqLengths, default to 0
    const currSeqLength = prevSeqLength + 1;

    seqLengths.set(currNum, currSeqLength);
    maxSeqLength = Math.max(maxSeqLength, currSeqLength);
  }

  return maxSeqLength;
}


/* old and unoptimized solution:
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
}*/