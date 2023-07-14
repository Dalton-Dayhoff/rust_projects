import requests
from bs4 import BeautifulSoup
import re
import toml

# Send an HTTP GET request to the website
# Orbital parameters
orbit_url = 'https://ssd.jpl.nasa.gov/planets/approx_pos.html'
# Physical parameters, 1 continues to indicate physical parameters
physical_url = 'https://ssd.jpl.nasa.gov/planets/phys_par.html'
response = requests.get(orbit_url)
response1 = requests.get(physical_url)

# Check if the request was successful
if response.status_code == 200:
    # Parse the HTML content using BeautifulSoup
    soup = BeautifulSoup(response.content, 'html.parser')
    soup1 = BeautifulSoup(response1.content, 'html.parser')
    
    # Find the table containing the data
    table = soup.find_all('table')
    table = table[1]
    table1 = soup1.find_all('table')
    table1 = table1[0]
    
    # Extract the table headers
    headers = [header.text.strip() for i, (header) in enumerate(table.find_all('td')) if i %2 != 0]
    # Physical data headers are only radius and mass
    headers1 = [header.text.strip() for i, (header) in enumerate(table1.find_all('thead')[0].contents[1].contents) if i % 2 != 0]
    headers1 = headers1[2:4]
    # Reformat the headers to be accurate of what they will be on the toml
    headers = [re.sub("\s+|-", "_", header).lower() for header in headers]
    for i, (header) in enumerate(headers):
        index = header.find('[')
        index2 = header.find(',')
        if index != -1:
            units = header[index+1:index2]
            match i:
                case 0: 
                    headers[i] = header[0:index] + "km"
                case _:
                    headers[i] = header[0:index] + units
    rows = []
    # Dictionary for mapping planets to headers/data
    data = {}
    # Extract the table rows
    # Define the regular expression pattern
    pattern = r'\b(\w+)\s+([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)\s+([\d.-]+)'
    # Extract the data using re.findall()
    stuff = soup.find_all('pre')[0].contents[0]
    lists_of_data = re.findall(pattern, stuff)
    base_data1 = [data.contents for i, (data) in enumerate(table1.find_all('tbody')[0].contents[:-1]) if i % 2 != 0]
    # fixing base_data1
    base_data1 = [[data[1].text.strip(), re.sub("\n|\s+","",data[5].text.strip()), re.sub("\n\s+","",data[7].text.strip())] for data in base_data1]
    indecies = [[planet_data[1].find('['), planet_data[2].find('[')] for planet_data in base_data1]
    physical_data = [[data[0], float(data[1][0:indecies[i][0]]), float(data[2][0:indecies[i][1]])] for i, (data) in enumerate(base_data1)]
    # Organize the data
    for i, (row_data) in enumerate(lists_of_data[:-1]):
        # Creating a dictionary from the headers to the values
        name_of_body = row_data[0] if i !=2 else "Earth"
        data[name_of_body] = {}
        # Remove everything that isnt a number, accounting for if data is not given
        new_data = [float(val) for val in row_data[1:]]
        # Convert AU to km
        new_data[0] = round(new_data[0]*149597870.7, 1)
        # Convert earth years to seconds
        # Filling in the dictionaries
        for j, (val) in enumerate(new_data):
            data[name_of_body][headers[j]] = val
        for j, (val) in enumerate(physical_data[i][1:]):
            if j == 0:
                data[name_of_body][headers1[j].lower() + "_km"] = val
            else:
                data[name_of_body][headers1[j].lower() + "_kg"] = val*(10**24)
        rows.append(row_data)
    
    # Define some more toml headers
    toml_dict = {"SolarSystem": data, "number_of_bodies": len(data)}

    # Print the table headers
    toml_str = toml.dumps(toml_dict)

    # Some headers have quotes around them, this removes those
    toml_str = re.sub(r'"([^"]*)"', r'\1', toml_str)
    
    # Print the table rows
    with open('data/celestial_bodies_data.toml', 'w') as file:
        file.write(toml_str)
else:
    print('Failed to retrieve data from the website.')