var N=null,E="",T="t",U="u",searchIndex={};
var R=["sorted_vec","option","result","try_from","borrow","type_id","typeid","borrow_mut","try_into","usize","sortedvec","formatter","SortedVec","A set based on a 2-level rotated array.","An iterator over the items of a `SortedVec`.","Makes a new `SortedVec` without any heap allocations.","Clears the set, removing all values.","contains","Returns `true` if the set contains a value.","Returns a reference to the value in the set, if any, that…","Returns a reference to the value in the set, if any, with…","Adds a value to the set.","Removes a value from the set. Returns whether the value…","Removes and returns the value in the set, if any, that is…","Returns the number of elements in the set.","is_empty","Returns `true` if the set contains no elements.","Gets an iterator that visits the values in the `SortedVec`…","to_owned","clone_into","into_iter","is_sorted","size_hint","next_back","nth_back","from_iter","default"];
searchIndex[R[0]]={"doc":E,"i":[[3,R[12],R[0],R[13],N,N],[3,"Iter",E,R[14],N,N],[11,"new",E,R[15],0,[[],["self"]]],[11,"clear",E,R[16],0,[[["self"]]]],[11,R[17],E,R[18],0,[[["self"],[T]],["bool"]]],[11,"get",E,R[19],0,[[["self"],[T]],[R[1]]]],[11,"at",E,R[20],0,[[["self"],[R[9]]],[R[1]]]],[11,"insert",E,R[21],0,[[["self"],[T]],["bool"]]],[11,"remove",E,R[22],0,[[["self"],[T]],["bool"]]],[11,"take",E,R[23],0,[[["self"],[T]],[R[1]]]],[11,"len",E,R[24],0,[[["self"]],[R[9]]]],[11,R[25],E,R[26],0,[[["self"]],["bool"]]],[11,"iter",E,R[27],0,[[["self"]],["iter"]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[28],E,E,0,[[["self"]],[T]]],[11,R[29],E,E,0,N],[11,R[3],E,E,0,[[[U]],[R[2]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[2]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[30],E,E,1,[[["self"]],["i"]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[3],E,E,1,[[[U]],[R[2]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[2]]]],[11,"from",E,E,0,N],[11,"from",E,E,0,[[["vec"]],["self"]]],[11,"next",E,E,1,[[["self"]],[R[1]]]],[11,"nth",E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"count",E,E,1,[[["self"]],[R[9]]]],[11,"last",E,E,1,[[["self"]],[R[1]]]],[11,"max",E,E,1,[[["self"]],[R[1]]]],[11,"min",E,E,1,[[["self"]],[R[1]]]],[11,R[31],E,E,1,[[["self"]],["bool"]]],[11,R[32],E,E,1,N],[11,"len",E,E,1,[[["self"]],[R[9]]]],[11,R[36],E,E,0,[[],[R[10]]]],[11,R[33],E,E,1,[[["self"]],[R[1]]]],[11,R[34],E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"clone",E,E,0,[[["self"]],[R[10]]]],[11,"fmt",E,E,0,[[["self"],[R[11]]],[R[2]]]],[11,"fmt",E,E,1,[[["self"],[R[11]]],[R[2]]]],[11,R[35],E,E,0,[[["i"]],["self"]]]],"p":[[3,R[12]],[3,"Iter"]]};
searchIndex[R[0]]={"doc":E,"i":[[3,R[12],R[0],R[13],N,N],[3,"Iter",E,R[14],N,N],[11,"new",E,R[15],0,[[],["self"]]],[11,"clear",E,R[16],0,[[["self"]]]],[11,R[17],E,R[18],0,[[["self"],[T]],["bool"]]],[11,"get",E,R[19],0,[[["self"],[T]],[R[1]]]],[11,"at",E,R[20],0,[[["self"],[R[9]]],[R[1]]]],[11,"insert",E,R[21],0,[[["self"],[T]],["bool"]]],[11,"remove",E,R[22],0,[[["self"],[T]],["bool"]]],[11,"take",E,R[23],0,[[["self"],[T]],[R[1]]]],[11,"len",E,R[24],0,[[["self"]],[R[9]]]],[11,R[25],E,R[26],0,[[["self"]],["bool"]]],[11,"iter",E,R[27],0,[[["self"]],["iter"]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[28],E,E,0,[[["self"]],[T]]],[11,R[29],E,E,0,N],[11,R[3],E,E,0,[[[U]],[R[2]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[2]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[30],E,E,1,[[["self"]],["i"]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[3],E,E,1,[[[U]],[R[2]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[2]]]],[11,"from",E,E,0,N],[11,"from",E,E,0,[[["vec"]],["self"]]],[11,"next",E,E,1,[[["self"]],[R[1]]]],[11,"nth",E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"count",E,E,1,[[["self"]],[R[9]]]],[11,"last",E,E,1,[[["self"]],[R[1]]]],[11,"max",E,E,1,[[["self"]],[R[1]]]],[11,"min",E,E,1,[[["self"]],[R[1]]]],[11,R[31],E,E,1,[[["self"]],["bool"]]],[11,R[32],E,E,1,N],[11,"len",E,E,1,[[["self"]],[R[9]]]],[11,R[36],E,E,0,[[],[R[10]]]],[11,R[33],E,E,1,[[["self"]],[R[1]]]],[11,R[34],E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"clone",E,E,0,[[["self"]],[R[10]]]],[11,"fmt",E,E,0,[[["self"],[R[11]]],[R[2]]]],[11,"fmt",E,E,1,[[["self"],[R[11]]],[R[2]]]],[11,R[35],E,E,0,[[["i"]],["self"]]]],"p":[[3,R[12]],[3,"Iter"]]};
searchIndex[R[0]]={"doc":E,"i":[[3,R[12],R[0],R[13],N,N],[3,"Iter",E,R[14],N,N],[11,"new",E,R[15],0,[[],["self"]]],[11,"clear",E,R[16],0,[[["self"]]]],[11,R[17],E,R[18],0,[[["self"],[T]],["bool"]]],[11,"get",E,R[19],0,[[["self"],[T]],[R[1]]]],[11,"at",E,R[20],0,[[["self"],[R[9]]],[R[1]]]],[11,"insert",E,R[21],0,[[["self"],[T]],["bool"]]],[11,"remove",E,R[22],0,[[["self"],[T]],["bool"]]],[11,"take",E,R[23],0,[[["self"],[T]],[R[1]]]],[11,"len",E,R[24],0,[[["self"]],[R[9]]]],[11,R[25],E,R[26],0,[[["self"]],["bool"]]],[11,"iter",E,R[27],0,[[["self"]],["iter"]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[28],E,E,0,[[["self"]],[T]]],[11,R[29],E,E,0,N],[11,R[3],E,E,0,[[[U]],[R[2]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[2]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[30],E,E,1,[[["self"]],["i"]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[3],E,E,1,[[[U]],[R[2]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[2]]]],[11,"from",E,E,0,N],[11,"from",E,E,0,[[["vec"]],["self"]]],[11,"next",E,E,1,[[["self"]],[R[1]]]],[11,"nth",E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"count",E,E,1,[[["self"]],[R[9]]]],[11,"last",E,E,1,[[["self"]],[R[1]]]],[11,"max",E,E,1,[[["self"]],[R[1]]]],[11,"min",E,E,1,[[["self"]],[R[1]]]],[11,R[31],E,E,1,[[["self"]],["bool"]]],[11,R[32],E,E,1,N],[11,"len",E,E,1,[[["self"]],[R[9]]]],[11,R[36],E,E,0,[[],[R[10]]]],[11,R[33],E,E,1,[[["self"]],[R[1]]]],[11,R[34],E,E,1,[[["self"],[R[9]]],[R[1]]]],[11,"clone",E,E,0,[[["self"]],[R[10]]]],[11,"fmt",E,E,0,[[["self"],[R[11]]],[R[2]]]],[11,"fmt",E,E,1,[[["self"],[R[11]]],[R[2]]]],[11,R[35],E,E,0,[[["i"]],["self"]]]],"p":[[3,R[12]],[3,"Iter"]]};
initSearch(searchIndex);addSearchOptions(searchIndex);