// the send trait can savely move ownership of a u32 to another thread. 
// raw pointer cant be safely moved to another thread since they have no safety guards. raw pointer can copy multiple times and it could happen that one gets to one thread and the other stays in the current one. what happen if both try to manipulate the same memory at the same time. it will create undefined behavior.
// referenced counted pointer or can call rc type. 
// the clone() method
// arc = the atomically reference counted pointer - use rc if you dont need to send a reference to another thread


