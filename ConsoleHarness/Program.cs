using System;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using RustflameFFI_CS;

namespace ConsoleHarness
{
    internal class Program
    {
        private static void Main(string[] args)
        { Stopwatch sw = new Stopwatch();
            sw.Start();
            Console.WriteLine("Hello from C#!");
            RustCalls.hello_world();
            int width = 5120, height = 5120;
            var ptr = RustCalls.ReturnStringUTF8();
            var str = Marshal.PtrToStringUni(ptr);
            Console.WriteLine(str);
            Console.WriteLine(sw.Elapsed);
            var imgptr = RustCalls.ReturnImageRG24BPP((uint)width, (uint)height);
            Console.WriteLine(sw.Elapsed);
            var bmp = new Bitmap(width, height,PixelFormat.Format24bppRgb);
            var imgarray = new byte [width * height * 3];
            Marshal.Copy(imgptr, imgarray, 0, (int) (width * height * 3));
            Console.WriteLine("Copied the memory over");
            Console.WriteLine(sw.Elapsed);
            byte r, g, b;
            for (var j = 0; j < height; j++)
            for (var i = 0; i < width; i++)
            {
                r = imgarray[((j * width + i)*3)];
                g = imgarray[((j * width + i)*3) + 1];
                b = 0;
                bmp.SetPixel(i, j, Color.FromArgb(r, g, b));
            }

            bmp.Save($"{DateTime.Now.ToFileTime()} rust rgout.bmp");
            Console.WriteLine(sw.Elapsed);
            sw.Stop();
            Console.WriteLine($"Wrote out an image {width}x{height} in size, generated in rust.");
            Console.ReadLine();
        }
    }
}