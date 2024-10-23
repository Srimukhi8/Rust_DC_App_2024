#include <iostream>
#include <vector>
using namespace std;
int bonus1(int arr[10][6],int over) {
    for(int i=0;i<6;i++) {
        if(arr[over][i]<4) {
            return 0;
        }
    }
    return 1;
}
int bonus2(int arr[10][6],int over) {
    if(arr[over][0]==6&&arr[over][1]==6) {
       return 2;
    } 
    else {
      return 0;
    }
}
int bonus3(int arr[10][6],int over) {
    if(arr[over][4]==6&&arr[over][5]==6) {
       return 3;
    } 
    else {
      return 0;
    }
}
int maiden(int arr[10][6],int over) {
    for(int i=0;i<6;i++) {
        if(arr[over][i]!=0) {
            return 0;
        }
    }
    return 4;
}
int main() {
    int arr[10][6];
    int max[10];
    int score=0;
    for(int i=0;i<10;i++) {
        max[i]=0;
        for(int j=0;j<6;j++) {
            cin >> arr[i][j];
            score+=arr[i][j];
            if(arr[i][j]>max[i]) {max[i]=arr[i][j];}
        }
    }
    int b[10]={0};
    vector<int> keep;
    for(int i=0;i<10;i++) {
        if(maiden(arr,i)==4) {
            b[i]=3;
            b[i+1]=0;
            i++;
        }
        else if(max[i]>=4) {
            if(bonus1(arr,i)==1) {
               b[i]=bonus1(arr,i);
            }
            else {
                b[i]=bonus2(arr,i)+bonus3(arr,i);
            }
        }
        switch(b[i]) {
            case 1: {
                for(int j=i+1;j<i+3&&j<10;j++) {
                    keep.push_back(j);
                    for(int k=0;k<6;k++) {
                        score+=arr[j][k];
                    }
                }
            }break;
            case 2: {
                for(int j=3;j<6;j++) {
                    score+=arr[i][j];
                    keep.push_back(i+10);
                }
            }break;
            case 3: {
                if(i!=9) {
                    for(int j=0;j<6;j++) {
                        score+=arr[i+1][j];
                    }
                    keep.push_back(i+1);
                }
            }break;
            case 5: {
                for(int j=3;j<6;j++) {
                    score+=arr[i][j];
                    keep.push_back(i+10);
                }
                if(i!=9) {
                    for(int j=0;j<6;j++) {
                        score+=arr[i+1][j];
                    }
                    keep.push_back(i+1);
                }
            }break;
        }
    }
    for(int i=0;i<10;i++) {
        int flag=0;
        for(int j=0;j<keep.size();j++) {
            if(keep[j]==i) {
               flag=1;
               break;
            }
        }
        for(int k=0;k<6;k++) {
            if(flag==1) {
                if(arr[i][k]>=4) {
                    cout << "*" << arr[i][k] << "X2 ";
                }
                else {
                    cout << arr[i][k] << "X2 ";
                }
            }
            else {
                if(arr[i][k]>=4) {
                    cout << "*" << arr[i][k] << " ";
                }
            }
        }
        cout << "\n";
    }
    if(b[8]==1||b[9]==3) {
        int extra[6];
        for(int i=0;i<6;i++) {
            cin >> extra[i];
            score+=2*extra[i];
            if(extra[i]>=4) {cout << "*" << extra[i] << "X2 ";}
            else {cout << extra[i] << "X2 ";}
        }
        cout << "\n";
    }
    else if(b[9]==1) {
        int extra1[6];
        int extra2[6];
        for(int i=0;i<6;i++) {
            cin >> extra1[i];
            score+=2*extra1[i];
            if(extra1[i]>=4) {cout << "*" << extra1[i] << "X2 ";}
            else {cout << extra1[i] << "X2 ";}
        }
        for(int i=0;i<6;i++) {
            cin >> extra2[i];
            score+=2*extra2[i];
            if(extra2[i]>=4) {cout << "*" << extra2[i] << "X2 ";}
            else {cout << extra2[i] << "X2 ";}
        }
        cout << "\n";
    }
    cout << score << "\n";
}