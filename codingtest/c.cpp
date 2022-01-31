#include<iostream>
#include<vector>
#include<set>
#include<
int main() {
	int n;
	std::cin >> n;
	std::set<int> st;
	for(int i = 0; i < n; i++) {
		int a;
		std::cin >> a;
		st.insert(a);
	}
	int k;
	std::cin >> k;
	std::cout << (st.count(k) ? "YES" :"NO") << std::endl;
}