#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <fstream>
#include <sstream>
#include <locale>

struct Line {
    std::string left;
    std::string right;

    Line(std::string l, std::string r) {
        this->left = l;
        this->right = r;
    }
};

Line split(std::string str, char del) {
    int del_index = -1;
    for (int i = 0; i < str.length(); i++) {
        if (str.at(i) == del) {
            del_index = i;
            break;
        }
    }

    Line l(str.substr(0, del_index), str.substr(del_index+1));

    return l;
}

enum CaveType {
    Normal,
    Start,
    End
};

class Cave {
protected:
    CaveType type;
    std::string name;
    std::vector<Cave*> connections;
public:
    Cave(std::string name, CaveType type);
    void add_connection(Cave* c);
    std::vector<Cave*> get_all_connections();
    std::string get_name();
};

Cave::Cave(std::string name, CaveType type) {
    this->name = name;
    this->type = type;
}

void Cave::add_connection(Cave* c) {
    this->connections.push_back(c);
}

std::vector<Cave*> Cave::get_all_connections() {
    return this->connections;
}

std::string Cave::get_name() {
    return this->name;
}

class SmallCave : public Cave {
private:
    bool visited;
public:
    SmallCave(std::string name, CaveType type) : Cave(name, type) {
        this->visited = false;
    }
    bool is_visited();
    void visit();
};

bool SmallCave::is_visited() {
    return visited;
}

void SmallCave::visit() {
    this->visited = true;
}

bool is_lower(std::string str) {
    for (int i = 0; i < str.length(); i++) {
        if (!std::islower(str.at(i))) {
            return false;
        }
    }

    return true;
}

void create_cave(std::map<std::string, Cave>* caves, std::string str) {
    if (caves->find(str) == caves->end()) {
        CaveType type;
        if (str == "start") {
            type = Start;
        } else if (str == "end") {
            type = End;
        } else {
            type = Normal;
        }

        if (is_lower(str)) {
            caves->insert(std::pair<std::string, Cave>(str, SmallCave(str, type)));
        } else {
            caves->insert(std::pair<std::string, Cave>(str, Cave(str, type)));
        }
    }
}

std::map<std::string, Cave> parse_input(bool test) {
    std::string file;
    if (test) {
        file = "../input/year2021/test12.txt";
    } else {
        file = "..input/year2021/day12.txt";
    }

    std::map<std::string, Cave> caves;

    std::ifstream infile(file);
    std::string line;
    while (std::getline(infile, line)) {
        Line l = split(line, '-');

        create_cave(&caves, l.left);
        create_cave(&caves, l.right);

        caves.find(l.left)->second.add_connection(&caves.find(l.right)->second);
        caves.find(l.right)->second.add_connection(&caves.find(l.left)->second);
    }

    return caves;
}

int main() {
    std::map<std::string, Cave> caves = parse_input(true);
    std::vector<Cave*> start = caves.find("start")->second.get_all_connections();

    for (int i = 0; i < start.size(); i++) {
        std::cout << start.at(i)->get_name() << "\n";
    }
    std::cout << std::endl;

    return 0;
}