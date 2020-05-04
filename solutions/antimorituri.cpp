#include <iostream>
#include <cstdio>
#include <cmath>
#include <vector>
#include <set>
#include <map>
#include <algorithm>
#include <string>
#include <cstring>

typedef long long ll;

using namespace std;

pair<int, int> addRC[4] = {{-1, 1}, {0, 1}, {1, 1}, {1, 0}};
ll SCORES[6] = {0, 1, 50, 1000, 1000 * 1000, 1000 * 1000 * 1000};
bool can[9][9];

ll getScore(const vector<string>& field) {
    ll totalScore = 0;
    for (int r = 0; r < 9; ++r)
        for (int c = 0; c < 9; ++c) {
            for (int type = 0; type < 4; ++type) {
                int lastR = r + addRC[type].first * 5;
                int lastC = c + addRC[type].second * 5;
                if (lastR >= 0 && lastR < 9 && lastC >= 0 && lastC < 9)
                {
                    vector<pair<char, pair<int, int> > > cur;
                    for (int i = 0; i < 5; ++i) {
                        int curR = r + addRC[type].first * i;
                        int curC = c + addRC[type].second * i;
                        if (field[curR][curC] != '_')
                            cur.push_back({field[curR][curC], {curR, curC}});
                    }
                    sort(cur.begin(), cur.end());
                    if (!cur.empty())
                    {
                        if (cur[0].first != cur[cur.size() - 1].first)
                            continue;

                        int sumDist = 0;
                        vector<pair<int, int> > onlyPos;
                        for (int i = 0; i < cur.size(); ++i) {
                            onlyPos.push_back(cur[i].second);
                        }
                        sort(onlyPos.begin(), onlyPos.end());
                        for (int i = 1; i < onlyPos.size(); ++i) {
                            sumDist += abs(onlyPos[i].first - onlyPos[i - 1].first) + abs(onlyPos[i].second - onlyPos[i - 1].second);
                        }

                        totalScore += (50 - sumDist) * 1ll * SCORES[(int)cur.size()];
                    }
                }
            }
        }
    return totalScore;
}

pair<int, int> q[100];

int main()
{
    //while (true) {
        vector <string> field(9);
        for (int i = 0; i < 9; ++i) {
            getline(cin, field[i]);
            string newF = "";
            for (int j = 0; j < field[i].size(); j+=2)
                newF += field[i][j];
            field[i] = newF;
        }
        int score;
        cin >> score;


        int bestR1, bestR2, bestC1, bestC2;
        ll bestScore = -1;

        for (int r1 = 0; r1 < 9; ++r1)
            for (int c1 = 0; c1 < 9; ++c1) {
                if (field[r1][c1] == '_')
                    continue;

                for (int i = 0; i < 9; ++i) {
                    for (int j = 0; j < 9; ++j) {
                        can[i][j] = false;
                    }
                }
                can[r1][c1] = true;
                q[0] = {r1, c1};
                int l = 0, r = 1;
                while (l < r) {
                    int curR = q[l].first;
                    int curC = q[l].second;
                    l++;

                    for (int addR = -1; addR <= 1; ++addR)
                        for (int addC = -1; addC <= 1; ++addC) {
                            if (addR * addC != 0)
                                continue;
                            if (addR == 0 && addC == 0)
                                continue;

                            int newR = curR + addR;
                            int newC = curC + addC;
                            if (newR >= 0 && newR < 9 && newC >= 0 && newC < 9 && field[newR][newC] == '_') {
                                if (!can[newR][newC]) {
                                    can[newR][newC] = true;
                                    q[r] = {newR, newC};
                                    r++;
                                }
                            }
                        }
                }

                /*for (int i = 0; i < 9; ++i) {
                    for (int j = 0; j < 9; ++j) {
                        if (can[i][j])
                            cout << "Y";
                        else
                            cout << ".";
                    }
                    cout << endl;
                }
                cout << endl; */

                for (int r2 = 0; r2 < 9; ++r2) {
                    for (int c2 = 0; c2 < 9; ++c2) {
                        if (!can[r2][c2])
                            continue;

                        if (r1 == r2 && c1 == c2)
                            continue;

                        char c = field[r1][c1];
                        field[r1][c1] = '_';
                        field[r2][c2] = c;

                        ll curScore = getScore(field);

                        /*if (r1 == 0 && c1 == 7 && r2 == 8 && c2 == 3)
                        {
                            cout << "OK" << endl;
                            cout << curScore << endl;
                            cout << bestScore << endl;
                        } */

                        if (curScore > bestScore) {
                            bestScore = curScore;
                            bestR1 = r1;
                            bestR2 = r2;
                            bestC1 = c1;
                            bestC2 = c2;
                        }

                        field[r1][c1] = c;
                        field[r2][c2] = '_';

                    }
                }
            }
        cout << bestC1 + 1 << " " << bestR1 + 1 << " " << bestC2 + 1 << " " << bestR2 + 1 << endl;
        //cout.flush();
    //}
    return 0;
}
