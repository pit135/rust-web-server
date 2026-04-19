[Commit 1 Reflection notes]

Pada tahap ini, saya buat server sederhana menggunakan Rust yang bisa terima koneksi dari browser dan membaca HTTP request yang dikirimkan. Saat server dijalankan pake cmd 'cargo run' dan browser mengakses `http://127.0.0.1:7878`, server terima beberapa request dan ditampilin di terminal

1. Cara Kerja Program
a. TcpListener
TcpListener::bind("127.0.0.1:7878")` dipake untuk:
- buka port 7878
- listen koneksi dari client (browser)

b. Incoming Connection
for stream in listener.incoming()

Loop ini akan:
- gunakan koneksi masuk
- setiap koneksi direpresentasikan sebagai TcpStream

c. Handle Connection
setiap koneksi dikirim ke fungsi:
handle_connection(stream);

2. Membaca HTTP Request
a. BufReader
let buf_reader = BufReader::new(&mut stream);
dipake untuk baca data dari stream
b. Baca per baris
.lines()
HTTP request dibaca baris per baris
c. Berhentiin pembacaan
.take_while(|line| !line.is_empty())
berfungsi untuk:
berhenti pas nemuin baris kosong, karna dalam HTTP, baris kosong menandakan akhir header

3. Struktur HTTP Request

Dari hasil output, request yang diterima formatnya seperti:

GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 ...
Accept: text/html ...

=> Penjelasan:
GET / HTTP/1.1 → request line
Host → alamat server
User-Agent → informasi browser
Accept → tipe konten yang bisa diterima browser

4. Observasi Hasil
Saat saya buka browser, muncul banyak output seperti:
Request: [ ... ]
Request: [ ... ]
Request: [ ... ]
....

Ini terjadi karena:
- browser kirim beberapa request secara otomatis
- retry kalau ga dapet response
- bisa juga karena fitur seperti keep alive

5. Warning pada Program
warning: unused variable: `stream`
Terjadi karena pada versi awal, variable stream ga dipake
Tapi setelah menggunakan handle_connection, warning ini ga lagi menjadi masalah

6. Kesimpulan
- browser komunikasi dengan server menggunakan HTTP request
- server baca request dalam bentuk teks
- rust bisa digunakan untuk membuat server sederhana menggunakan TcpListener dan TcpStream
- HTTP request terdiri dari beberapa baris yang diakhiri dengan baris kosong
- satu kali akses browser ga selalu hasilin satu request aja  
  

[Commit 2 Reflection notes]

Pada milestone ini, saya belajar bagaimana web server sederhana di Rust bisa  mengembalikan (return) halaman HTML ke browser
Fungsi handle_connection untuk tangani setiap koneksi yang masuk dari client (browser). Pertama, program baca HTTP request pake BufReader dan ambil baris-baris request sampai temukan baris kosong yang menandakan akhir header, selanjutnya, server buat response HTTP secara manual
Response ini terdiri dari:
- status line: HTTP/1.1 200 OK
- header: Content-Length, yang menunjukkan panjang konten HTML
- body: isi dari file hello.html  
File HTML dibaca pake fs::read_to_string, lalu panjangnya diitung untuk dimasukkan ke header. Terakhir, response yang udah diformat dikirim lagi ke browser pake stream.write_all(). Browser lalu merender isi HTML tersebut sehingga bisa ditampilkan sebagai halaman web 

Dari percobaan ini, saya memahami bahwa komunikasi antara browser dan server pake protokol HTTP, dan server harus mengirim response pake format yang benar supaya browser bisa nampilin dengan baik

![commit 2 screen capture - terminal](images/terminal.png)
![Commit 2 screen capture - browser (html)](images/browser.png)