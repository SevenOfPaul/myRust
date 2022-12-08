use std::{mem, ptr, slice};
use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ops::{Deref, DerefMut};
use std::path::Iter;
use std::ptr::NonNull;
#[derive(Debug)]
struct Raw<T>{
    ptr: NonNull<T>,
    cap: usize,
}
impl<T> Raw<T>{
    fn new()->Self{
        let cap=if mem::size_of::<T>()==0{!0}else{0};
        assert_ne!(mem::size_of::<T>(), 0, "零尺寸类型无法处理");
        Raw{
            ptr:NonNull::dangling(),
            cap:cap
        }
    }
    //内存分配
    fn grow(&mut self) {
        unsafe {
            // 因为当 T 的尺寸为 0 时我们设置了 cap 为 usize::MAX
            // 这一步成立意味着 Vec 溢出了
            assert!(mem::size_of::<T>() != 0, "capacity overflow");
            //获取到类型对齐方式
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let layout: Layout;
            let (new_cap, ptr) = if self.cap == 0 {
                layout = Layout::from_size_align_unchecked(elem_size, align);
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = self.cap * 2;
                let old_num_bytes = self.cap * elem_size;
                assert!(old_num_bytes <= (isize::MAX as usize) / 2, "容量溢出");
                let new_num_bytes = old_num_bytes * 2;
                layout = Layout::from_size_align_unchecked(new_num_bytes, align);
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_num_bytes);
                (new_cap, ptr)
            };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }
            if let Some(ptr) = NonNull::new(ptr as *mut _) {
                self.ptr = ptr;
                self.cap=new_cap;
            } else {
                panic!("error");
            }
        }
    }
}
impl<T> Drop for Raw<T> {
    fn drop(&mut self) {
        let ele_size=mem::size_of::<T>();
        if self.cap!= 0&&ele_size!=0 {
            //取出元素
            //回收内存
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe {
                let layout=Layout::from_size_align_unchecked(num_bytes, align);
                dealloc(self.ptr.as_ptr() as *mut u8,layout);
            }
        }
        println!("内存释放");
    }
}
#[derive(Debug)]
struct MVec<T> {
    raw: Raw<T>,
    len: usize,
}
impl<T> MVec<T> {
    fn new() -> Self {
        MVec {
            raw: Raw::new(),
            len: 0,
        }
    }
    fn ptr(&self)->*mut T{
        self.raw.ptr.as_ptr()
    }
    fn cap(&self)->usize{
        self.raw.cap
    }
    unsafe fn push(&mut self, val: T) {
        if self.len == self.cap() {
            self.raw.grow()
        }
        unsafe {
            ptr::write(self.ptr().offset(self.len as isize), val);
        }

        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().offset(self.len as isize))) }
        }
    }
    fn get(&self, index: usize) -> Option<T> {
        if index < self.len {
            unsafe {
                //这里count会直接获取到size
                Some(ptr::read(self.ptr().offset(index as isize)))
            }
        } else {
            None
        }
    }
    //----------------
    fn insert(&mut self, pos: usize, val: T) {
        assert!(pos < self.len, "下标越界");
        if self.cap() == self.len {
            self.raw.grow();
        }
        unsafe {
            ptr::copy(
                //移动哪里的内存
                self.ptr().offset(pos as isize),
                //移动到内存
                self.ptr().offset((pos + 1) as isize),
                self.len - pos as usize,
            );
            ptr::write(self.ptr().offset(pos as isize), val);
        }
        self.len += 1;
    }
    fn remove(&mut self, pos: usize) {
        assert!(pos >= 0 || pos < self.len, "下标越界");
        unsafe {
            ptr::copy(
                //移动哪里的内存
                self.ptr().offset((pos + 1) as isize),
                //移动到内存
                self.ptr().offset(pos as isize),
                self.len - pos as usize,
            );
            self.len -= 1;
        }
    }
    unsafe fn into_iter(self) ->IntoIter<T>{
        let ptr=ptr::read(&self.raw);
        let cap=self.cap();
        let len=self.len;
        mem::forget(self);
        unsafe {
            IntoIter{
                start:ptr.ptr.as_ptr(),
                end:if mem::size_of::<T>()==0 {
                    (ptr.ptr.as_ptr() as usize +len) as *const _
                }else if(cap==0){
                    ptr.ptr.as_ptr()
                }else{
                    ptr.ptr.as_ptr().add(len)
                },
                _buf:ptr
            }
        }
    }
    //-----------------
}
impl<T> Deref for MVec<T> {
    //切片类型
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr(), self.len) }
    }
}
impl<T> DerefMut for MVec<T> {
    // type Target =  [T];
    //切片类型
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}
#[derive(Debug)]
struct IntoIter<T> {
    _buf: Raw<T>,
    start: *const T,
    end: *const T,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
            for _ in &mut *self {}
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
            if mem::size_of::<T>()==0{
                    self.start=  (self.start as usize+1) as *const _;
                Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                }else{
                self.start =self.start.offset(1);
                Some(result)
                }
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let ele_size=mem::size_of::<T>();
        let len=((self.end as usize-self.start as usize)/if ele_size==0{1}else{ele_size});
        (len,Some(len))
    }
}
impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        }else {
            unsafe {
                //向前回滚一位
               if mem::size_of::<T>()==0{
                   self.end = (self.end as usize -1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                }else{
                   self.end =self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}
fn main() {
    let mut vec: MVec<i32> = MVec::new();
    unsafe {
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        let v = &mut vec[0..1];
        // println!("{:?}",vec.into_iter());
        let iter=vec.into_iter();
        for v in iter.rev(){
            println!("{}",v);
        }
        // drop(vec);
    }
}
