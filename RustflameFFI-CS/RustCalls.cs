using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace RustflameFFI_CS
{
    public static class RustCalls
    {
        [DllImport("rustflame_ffi.dll")]

        public static extern void hello_world();

        [DllImport("rustflame_ffi.dll",CallingConvention = CallingConvention.Cdecl)]
        public static unsafe extern void* return_string_utf8();

        [DllImport("rustflame_ffi.dll", CallingConvention = CallingConvention.Cdecl)]
        public static unsafe extern void* return_image_rg_24bpp(UInt32 width, UInt32 height);

        public static IntPtr ReturnStringUTF8()
        {
            IntPtr ret;
            unsafe { ret = new IntPtr(return_string_utf8());}
            return ret;
        }

        public static IntPtr ReturnImageRG24BPP(uint width, uint height)
        {
            IntPtr ret;
            unsafe { ret = new IntPtr(return_image_rg_24bpp(width,height));}

            return ret;
        }
}
}
