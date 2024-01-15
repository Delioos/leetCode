function findGoodStrings(n: number, s1: string, s2: string, evil: string): number {
    let arr = [];
    let currentString = s1;
    let done = false;
 
    while (!done) {
        if (!currentString.includes(evil)) {
            arr.push(currentString);
        }
 
        // Increment currentString
        let carry = 1;
        for (let i = n - 1; i >= 0 && carry != 0; i--) {
            let asciiVal = currentString.charCodeAt(i);
            if (asciiVal + carry > 'z'.charCodeAt(0)) {
                currentString = currentString.substring(0, i) + 'a' + currentString.substring(i + 1);
                carry = 1;
            } else {
                currentString = currentString.substring(0, i) + String.fromCharCode(asciiVal + carry) + currentString.substring(i + 1);
                carry = 0;
            }
        }
 
        // Check if we have reached s2
        if (carry != 0 || currentString > s2) {
            done = true;
        }
    }
 
    return arr.length % (10**9 + 7);
 }
 