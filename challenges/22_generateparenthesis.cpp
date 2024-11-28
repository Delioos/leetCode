class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> result;
        backtrack(result, "", 0, 0, n);
        return result;
    }
    
private:
    void backtrack(vector<string>& result, string current, int open, int close, int max) {
        // Base case: if current string length equals 2*n
        if (current.length() == max * 2) {
            result.push_back(current);
            return;
        }
        
        // If we can add an open parenthesis
        if (open < max) {
            backtrack(result, current + "(", open + 1, close, max);
        }
        
        // If we can add a closing parenthesis
        if (close < open) {
            backtrack(result, current + ")", open, close + 1, max);
        }
    }
};