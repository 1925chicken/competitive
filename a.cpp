#include <iostream>
#include <string>
#include<map>
#include<queue>
#include<vector>
using namespace std;

void step1() {//step1の実装
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
	vector<string> ans1;
	vector<int> ans2,ans3; 
	while(!cin.eof()) {
		string order;
		cin >> order;
		int T,D,N;
		cin >> T >> D >> N;

		if(number_of_menu_and_storage[D] < N) {
			//cout << "sold out " << T << "\n";
			ans1.emplace_back("sold out");
			ans2.emplace_back(T);
			ans3.emplace_back(-1);
		}else if(number_of_menu_and_storage[D] >= N){
			for(int i = 0; i < N; i++) {
				ans1.emplace_back("received order");
				ans2.emplace_back(T);
				ans3.emplace_back(D);
			//cout << "received order " << T << " " << D <<  "\n";
			}
			number_of_menu_and_storage[D] -= N;
		}
	}
	for(size_t i = 0; i < ans1.size(); i++) {
		cout << ans1[i] << " " << ans2[i];
		if(ans3[i] == -1) {
			cout << endl;
		}else {
			cout << " " << ans3[i] << endl;
		}
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
				ans1.emplace_back("wait");
				waiting_list.push(D);
			}else if(K > oven_used_now) {
				ans1.emplace_back("ee");
				ans2.emplace_back(D);
				//cout << D << "\n";
				oven_used_now += 1;
				howmany_cooked[D] += 1;
			}
		}else if (order == "complete") {
			int D;
			cin >> D;
			if(howmany_cooked[D] == 0) {
				ans1.emplace_back("unexpected input");
				//cout << "unexpected input\n";
			}else {
				//cout << "ok";
				ans1.emplace_back("ok");
				if(waiting_list.size() != 0) {
					howmany_cooked[D] -= 1;
					int now = waiting_list.front();
					waiting_list.pop();
					//cout << " " << now;
					ans2.emplace_back(now);
					howmany_cooked[now] += 1;
				}else {
					howmany_cooked[D] -= 1;
					oven_used_now -= 1;
					ans2.emplace_back(-1);
				}
				//cout << endl;
			}
		}
	}
	size_t i = 0;
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
	vector<queue<int>> ready(10001);
	vector<string> ans1;
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
			ans1.emplace_back("ready");
			ans2.emplace_back(ready[D].front());
			ans3.emplace_back(D);
			//cout << "ready " << ready[D].front() << " " << D << endl;
			ready[D].pop();
		}
	}
	for(size_t i = 0; i < ans1.size(); i += 1) {
		cout << ans1[i] << " " << ans2[i] << " " << ans3[i] << "\n";
	}
}
void step4() {

}

int main() {
	 int step;
	 cin >> step;
	 if (step == 1) step1();
	 else if(step == 2) step2();
	 else if (step == 3) step3();
	 else step4();
}