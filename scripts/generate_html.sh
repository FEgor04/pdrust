EXAMPLES=(fixed_length_bridge fixed_length_pendulum leq_length_pendulum pseudo_string pulley springs_double_pendulum)
echo "<!doctype html>" > ./out/examples/index.html
echo "<html>" >> ./out/examples/index.html
echo "<head>" >> ./out/examples/index.html
echo "</head>" >> ./out/examples/index.html
echo "<body>" >> ./out/examples/index.html
echo "<ul>" >> ./out/examples/index.html
for example in ${EXAMPLES[*]}
do
  echo "<li><a href='./$example/index.html'>$example</a></li>" >> ./out/examples/index.html
done
echo "</ul>" >> ./out/examples/index.html
echo "</body>" >> ./out/examples/index.html
echo "</html>" >> ./out/examples/index.html

for example in ${EXAMPLES[*]}
do
echo "<!doctype html>" > ./out/examples/$example/index.html
echo "<html lang='en'>" >> ./out/examples/$example/index.html
echo "<body style='margin: 0 auto;'>" >> ./out/examples/$example/index.html
echo "<script type='module'>" >> ./out/examples/$example/index.html
echo "import init from './$example.js'" >> ./out/examples/$example/index.html
echo "document.addEventListener('contextmenu', event => event.preventDefault());" >> ./out/examples/$example/index.html
echo "init().catch((error) => {" >> ./out/examples/$example/index.html
echo "if (!error.message.startsWith('Using exceptions for control flow, don\'t mind me. This isn\'t actually an error!')) {" >> ./out/examples/$example/index.html
echo "throw error;" >> ./out/examples/$example/index.html
echo "}" >> ./out/examples/$example/index.html
echo "});" >> ./out/examples/$example/index.html
echo "</script>" >> ./out/examples/$example/index.html
echo "</body>" >> ./out/examples/$example/index.html
echo "</html>" >> ./out/examples/$example/index.html
done
