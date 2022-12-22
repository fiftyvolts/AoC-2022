#!/usr/bin/perl
use strict;
use Data::Dumper;

$Data::Dumper::Indent = 0;
$Data::Dumper::Terse = 1;
local $" = ",";
my $i = 0;

my @nums = map {
    chomp $_;
    if ( $_ ne "" ) { [ $i++, $_ ] }
    else            { () }
} (<ARGV>);

my $size   = scalar @nums;
my $factor = $ENV{FACTOR} || 1;
my $rounds = $ENV{ROUNDS}             || 1;
my @nums   = map { [ $_->[0], $_->[1] * $factor ] } @nums;

print "0: ", Dumper( \@nums ), "\n" if $ENV{DEBUG} > 1;

my $next = 0;
my $idx  = 0;
for my $z (0..($rounds - 1)) {
 $next = 0;
 $idx  = 0;
 print "round $z: ", Dumper( \@nums ), "\n" if $ENV{DEBUG} >=1;
    while ( $next < @nums ) {
        my $t = $nums[$idx];

        if ( $t->[0] != $next ) {
            $idx++;
            next;
        }

        if ( $idx == 0 ) {
            @nums = @nums[ 1 .. $#nums ];
        }
        elsif ( $idx == $#nums ) {
            @nums = @nums[ 0 .. $idx - 1 ];
        }
        else {
            @nums = ( @nums[ 0 .. $idx - 1 ], @nums[ $idx + 1 .. $#nums ] );
        }

        my $nidx;
        if ( $t->[1] >= 0 ) {
            $nidx = ( $idx + $t->[1] ) % @nums;
        }
        else {
            $nidx = ( ( $idx + $t->[1] ) % @nums );
        }

        if ( $nidx == 0 ) {
            @nums = ( $t, @nums );
        }
        elsif ( $nidx == $#nums ) {
            @nums = ( @nums, $t );
        }
        else {
            @nums = ( @nums[ 0 .. $nidx - 1 ], $t, @nums[ $nidx .. $#nums ] );
        }

        print "$next: ", Dumper( \@nums ), " - moved idx: $idx, ndx: $nidx, dat: $t->[1]]\n"
          if $ENV{DEBUG} >= 2;

        $next++;
        $idx = 0;
    }
    die "didn't shuffle all numbers" if $next != @nums;
}

my $zero;
for $idx ( 0 .. $#nums ) {
    if ( $nums[$idx]->[1] == 0 ) {
        $zero = $idx;
    }
}
die "couldn't find zero" if !defined($zero);
print "zero at $zero\n";

my @final_idx = map { ( $_ + $zero ) % @nums } ( 1000, 2000, 3000 );
my @output    = map { $nums[$_]->[1] } @final_idx;
print "@final_idx -> @output ", $output[0] + $output[1] + $output[2], "\n";