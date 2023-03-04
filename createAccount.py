import sys

website = f"""
<!DOCTYPE html>
<html>
<head>
<title>Website</title>
</head>
<body>
<h1>Website</h1>
<p>Welcome to {sys.argv[1]}</p>
</body>
</html>
"""

print(website)