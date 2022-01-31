#pragma GCC target("avx2")
#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include<iostream>
#include<cstdint>
#include<queue>
#include<cstddef>
#include<map>
#include<array>
#include<vector>
#if __has_include(<atcoder/all>)
#include<atcoder/all>
using namespace atcoder;
#endif
//#include<boost/multiprecision/cpp_int.hpp>
//namespace mp = boost::multiprecision;
using namespace std;
using i32 = int_fast32_t;
using i64 = int_fast64_t;
using usize = size_t;
using u32 = uint_fast32_t;
using u64 = uint_fast64_t;
using i128 = __int128_t;
using u128 = __uint128_t;
using ld = long double;
template<typename T>
using vec = vector<T>;
#define rep(i, n) for (i64 i = 0; i < (i64)(n); ++i)
#define all(a) (a).begin(),(a).end()
#define rall(a) (a).rbegin(),(a).rend()
#define len(hoge) (i64)((hoge).size())
using P = pair<i64,i64>;
template<class T, class S> ostream &operator<<(ostream &os, const pair<T,S> &p){
	os << p.first << " " << p.second;
	return os;
}
void step1(){
	int m;
	cin >> m;
	map<int,int> number_of_menu_and_storage;
	for(int i = 0; i < m; i++) {
		int D,S,P;
		cin >> D >> S >> P;
		number_of_menu_and_storage[D] = S;
	}
	vector<string> ans;
	while(!cin.eof()) {
		string order;
		int T,D,N;
		cin >> order >> T >> D >> N;
		if(number_of_menu_and_storage[D] < N) {
			//cout << "sold out " << T << "\n";
			string a = "sold out ";
			a += to_string(T);
			ans.emplace_back(a);
		}else if(number_of_menu_and_storage[D] >= N){
			number_of_menu_and_storage[D] -= N;
			string a = "received order ";
			a += to_string(T);
			a += " ";
			a += to_string(D);
			for(int i = 0; i < N; i++) {
				ans.emplace_back(a);
				
			//cout << "received order " << T << " " << D <<  "\n";
			}
			//number_of_menu_and_storage[D] -= N;
		}
	}
	//cout << (len(ans)) << endl;
	for(int i = 0; i < len(ans) - 1; i++) {

		cout << ans[i] << endl;
	}
}
void step2() {
int M,K;
	cin >> M >> K;
	map<int,int> number_of_menu_and_storage;
	map<int,int> price_of_menu;
	vector<string> ans1;
	vector<int> ans2; 
	for(int i = 0; i < M; i++) {
		int D,S,P;
		cin >> D >> S >> P;
		number_of_menu_and_storage[D] = S;
		price_of_menu[D] = P;
	}
	string order;
	queue<int> waiting_list;
	int oven_used_now = 0;//今何個レンジを使っているか
	map<int,int> howmany_cooked;//現在どれがどれだけ調理されているか
	vector<string> ans;
	while(!cin.eof()) {
		cin >> order;
		if (order == "received") {
			string tmp;
			cin >> tmp;
			order += " ";
			order += tmp;
		}
		if (order == "received order") {
			int T,D;
			cin >> T >> D;
			if(K == oven_used_now) {
				//cout << "wait\n";
				ans.emplace_back("wait");
				waiting_list.push(D);
			}else if(K > oven_used_now) {
				/*ans1.emplace_back("ee");
				ans2.emplace_back(D);*/
				ans.emplace_back(to_string(D));
				//cout << D << "\n";
				oven_used_now += 1;
				howmany_cooked[D] += 1;
			}
		}else if (order == "complete") {
			int D;
			cin >> D;
			if(howmany_cooked[D] == 0) {
				ans.emplace_back("unexpected input");
				//cout << "unexpected input\n";
			}else {
				//cout << "ok";
				string a = "ok";
				if(waiting_list.size() != 0) {
					howmany_cooked[D] -= 1;
					int now = waiting_list.front();
					waiting_list.pop();
					//cout << " " << now;
					//ans2.emplace_back(now);
					a += " ";
					a += to_string(now);
					howmany_cooked[now] += 1;
				}else {
					howmany_cooked[D] -= 1;
					oven_used_now -= 1;
					//ans2.emplace_back(-1);
				}
				ans.emplace_back(a);
				//cout << endl;
			}
		}
	}
	/*size_t i = 0;
	size_t j = 0;
	while (i < ans1.size() && j < ans2.size()) {
		if(ans1[i] == "wait") {
			cout << ans1[i] << "\n";
			i += 1;
		}else if (ans1[i] == "ee") {
			cout << ans2[j] << "\n";
			i += 1;
			j += 1;
		}else if (ans1[i] == "ok") {
			cout << ans1[i];
			if(ans2[j] == -1) {
				cout << "\n";
			}else {
				cout << " " << ans2[j] << "\n";
			}
			i += 1;
			j += 1;
		}else if (ans1[i] == "unexpected input") {
			cout << ans1[i] <<"\n";
			i += 1;
		}
	}*/
	for(int i = 0; i < len(ans) - 1; i += 1) {
		cout << ans[i] << endl;
	} 
}
void step3() {
	int M;
	cin >> M;
	map<int,int> number_of_menu_and_storage;
	map<int,int> price_of_menu;
	for(int i = 0; i < M; i++) {
		int D,S,P;
		cin >> D >> S >> P;
		number_of_menu_and_storage[D] = S;
		price_of_menu[D] = P;
	}
	string order;
	int T,D;
	array<queue<int>,1000001>ready;
	vector<string> ans1;
	vector<string> ans;
	vector<int> ans2,ans3;
	while(!cin.eof()) {
		cin >> order;
		if(order == "received") {
			string tmp;
			cin >> tmp;
			order += " ";
			order += tmp;
		}
		if (order == "received order") {
			cin >> T >> D;
			ready[D].push(T);
		}else if(order == "complete") {
			cin >> D;
			string a ="ready ";
			a += to_string(ready[D].front());
			a += ' ';
			a += to_string(D);
			/*ans1.emplace_back("ready");
			ans2.emplace_back(ready[D].front());
			ans3.emplace_back(D);*/
			ans.emplace_back(a);
			//cout << "ready " << ready[D].front() << " " << D << endl;
			ready[D].pop();
		}
	}
	for(int i = 0; i < ans.size(); i += 1) {
		cout << ans[i] << endl;
	}
}
int main(){
	ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	int step;
	cin >> step;
	if (step == 1) step1();
	if (step == 2) step2();
	if (step == 3) step3();
}
