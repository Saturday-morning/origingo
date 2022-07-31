#  This program is free software; you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation; either version 2 of the License, or
#  (at your option) any later version.
#  
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#  
#  You should have received a copy of the GNU General Public License
#  along with this program; if not, write to the Free Software
#  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
#  MA 02110-1301, USA.
#  

name = "OriginGo"
print('2022年8月，gtp测试程序')
print('针对genmove和play命令使用了上网查找到的切片，将命令切成列表，方便调用')
while True:
	print("")
	gtp = input()
	if gtp == "name":
		print('=', name)
	elif gtp == "protocol_version":
		print('= 2')
	elif gtp == "list_commands":
		print('=')
		print('name\nprotocol_version\nlist_commands\nversion\nplay\ngenmove\nquit')
	elif gtp == "version":
		print('= 2')
	elif gtp[:5] =="play ":
		play = gtp.split(" ")
		print('=', play)
	elif gtp[:8] =="genmove ":
		genmove = gtp.split(" ")
		print('=', genmove)
	elif gtp == "quit":
		print('=')
		break
	else:
		print("? unknown kommand")

